pub use self::subscriber::{Subscriber, SubscriberHandle};
pub use async_nats::Subject;

use super::{debug, Message, StatusCode};
use crate::Error;
use dashmap::{mapref::multiple::RefMulti, DashMap, DashSet};
use ractor::{Actor, ActorProcessingErr, ActorRef};
use rand::{thread_rng, RngCore};
use std::{
    fmt,
    sync::{atomic, Arc},
};

type Pattern = Subject;
/// (`server_sid`, `client_sid`,`)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SubscriberId {
    Client(u64, u64),
    Temp(u64, u64),
}
impl SubscriberId {
    pub const fn cid(&self) -> u64 {
        match self {
            Self::Client(cid, _) => *cid,
            Self::Temp(cid, _) => *cid,
        }
    }
    pub fn to_parts(&self) -> (u64, u64) {
        match self {
            Self::Client(cid, sid) => (*cid, *sid),
            Self::Temp(cid, sid) => (*cid, *sid),
        }
    }
}
impl AsRef<SubscriberId> for SubscriberId {
    fn as_ref(&self) -> &SubscriberId {
        self
    }
}
impl From<(u64, u64)> for SubscriberId {
    fn from((cid, sid): (u64, u64)) -> Self {
        Self::Client(cid, sid)
    }
}
impl fmt::Display for SubscriberId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Client(cid, sid) => write!(f, "{cid}-s{sid}"),
            Self::Temp(cid, sid) => write!(f, "{cid}-S{sid}"),
        }
    }
}

///
pub type SubscriptionId = (Pattern, QueueGroup);

pub type QueueGroup = Option<String>;
pub type WeightedQueueGroup = Option<(String, Option<u32>)>;

/// Shared state of all active pubsub channels (client + cluster pubs and subs).
#[derive(Debug, Clone, Default)]
pub struct Relay {
    inner: Arc<RelayState>,
}

#[derive(Debug)]
struct RelayState {
    prefix: String,

    /// A map of [`SubscriberId`]s to their sender handles.
    subscribers: DashMap<SubscriberId, SubscriberHandle>,

    /// A map of subject patterns to sets of [`SubscriberId`]s and their queue group (if any).
    subscriptions: DashMap<SubscriptionId, DashSet<SubscriberId>>,

    ///
    last_subscriber_id: atomic::AtomicU64,
}

impl Default for RelayState {
    fn default() -> Self {
        Self {
            prefix: Default::default(),
            subscribers: Default::default(),
            subscriptions: Default::default(),
            last_subscriber_id: atomic::AtomicU64::new(1),
        }
    }
}

impl Relay {
    pub fn with_prefix(prefix: String) -> Self {
        Self {
            inner: Arc::new(RelayState {
                prefix,
                ..Default::default()
            }),
        }
    }

    pub fn publish<T: ractor::Message + From<(SubscriberId, Message)>>(
        &self,
        cid: u64,
        message: Message,
        receiver: ActorRef<T>,
        echo: bool,
    ) -> Result<StatusCode, Error> {
        // if reply_to, spawn subscriber
        if let Some(ref reply) = message.reply {
            let sub_id = self.next_temp_id(cid);
            let reply = reply.clone();
            let this = self.clone();
            tokio::task::spawn(async move {
                let _ = this.subscribe(sub_id, reply, None, receiver).await?;
                Ok::<(), Error>(())
            });
        }

        // find subscribers by matching their pattern against subject
        let subs = self
            .filter_subscribers(&message.subject)
            .filter(|id| echo || id.cid() != cid)
            .filter_map(|id| self.subscriber(id))
            .collect::<Vec<_>>();

        tracing::trace!("found {} subscribers", subs.len());

        // collect failures and stop them
        let mut status = StatusCode::NO_RESPONDERS;
        for handle in subs {
            match handle.cast(message.clone().into()) {
                Ok(_) => {
                    status = StatusCode::OK;
                }
                Err(_) => {
                    self.remove_subscriber(handle.as_ref());
                }
            };
        }

        Ok(status)
    }

    /// Gets or creates a [`Subscriber`] that routes published messages to the `receiver` actor.
    pub async fn subscribe<T: ractor::Message + From<(SubscriberId, Message)>>(
        &self,
        id: impl Into<SubscriberId>,
        pattern: Pattern,
        queue_group: QueueGroup,
        receiver: ActorRef<T>,
    ) -> Result<StatusCode, Error> {
        // spawn (or get?) subscriber that casts to receiver
        let id = id.into();
        let sub = Subscriber {
            id,
            relay: self.clone(),
            receiver,
        };
        let _ = Actor::spawn(Some(sub.name()), sub, (pattern, queue_group)).await?;

        Ok(StatusCode::OK)
    }

    pub fn unsubscribe(
        &self,
        id: impl Into<SubscriberId>,
        max_msgs: Option<u64>,
        reason: Option<String>,
    ) -> Result<StatusCode, Error> {
        let id = id.into();
        match (self.subscriber(id), max_msgs) {
            (None, _) => {}
            (Some(handle), Some(max_msgs)) => {
                handle.cast(max_msgs.into())?;
            }
            (Some(handle), None) => {
                handle.stop(reason);
            }
        };

        Ok(StatusCode::OK)
    }
}

impl Relay {
    fn next_temp_id(&self, cid: u64) -> SubscriberId {
        let sid = self
            .inner
            .last_subscriber_id
            .fetch_add(1, atomic::Ordering::Relaxed);
        SubscriberId::Temp(cid, sid)
    }

    /// Filters subscriptions by subject, then candidate subscribers by queue group.
    fn filter_subscribers<'a>(
        &'a self,
        subject: &'a Subject,
    ) -> impl Iterator<Item = SubscriberId> + 'a {
        self.filter_subscriptions(subject).flat_map(move |e| {
            enum Iter<I> {
                Single(Option<SubscriberId>),
                Multi(I),
            }
            impl<I: Iterator<Item = SubscriberId>> Iterator for Iter<I> {
                type Item = SubscriberId;
                fn next(&mut self) -> Option<Self::Item> {
                    match self {
                        Self::Single(id) => id.take(),
                        Self::Multi(iter) => iter.next(),
                    }
                }
                fn size_hint(&self) -> (usize, Option<usize>) {
                    match self {
                        Self::Single(id) => (id.is_some() as usize, Some(id.map_or(0, |_| 1))),
                        Self::Multi(iter) => iter.size_hint(),
                    }
                }
            }

            let ids = e.value().iter().map(|id| *id).collect::<Vec<_>>();
            let idx = thread_rng().next_u32() as usize % ids.len();
            match e.key().1 {
                Some(_) => Iter::Single(ids.get(idx).copied()),
                None => Iter::Multi(ids.into_iter()),
            }
        })
    }

    fn filter_subscriptions<'a>(
        &'a self,
        subject: &'a Subject,
    ) -> impl Iterator<Item = RefMulti<'a, SubscriptionId, DashSet<SubscriberId>>> + 'a {
        tracing::trace!(
            "filtering subscriptions ({})",
            self.inner.subscriptions.len()
        );
        self.inner
            .subscriptions
            .iter()
            .filter(move |e| subject.matches(&e.key().0) && !e.is_empty())
    }

    fn subscriber(&self, id: SubscriberId) -> Option<SubscriberHandle> {
        tracing::trace!("getting subscriber ({})", self.inner.subscribers.len());
        self.inner.subscribers.get(&id).map(|v| v.clone())
    }

    fn add_subscriber(
        &self,
        id: SubscriberId,
        handle: SubscriberHandle,
        pattern: Pattern,
        queue_group: QueueGroup,
    ) {
        self.inner.subscribers.insert(id, handle);
        self.inner
            .subscriptions
            .entry((pattern, queue_group))
            .or_insert_with(|| DashSet::with_capacity(3))
            .value_mut()
            .insert(id);
    }

    fn remove_subscriber(&self, id: impl AsRef<SubscriberId>) {
        let id = id.as_ref();
        let (_, _handle) = self.inner.subscribers.remove(id).unwrap();
        self.inner.subscriptions.iter().for_each(|e| {
            e.value().remove(id);
        });

        let (cid, sid) = id.to_parts();
        tracing::trace!(
            "{prefix} - {arrow} [{ctrl} {sid}]",
            prefix = debug::trace_prefix("xxx.xxx.xxx.xxx:yyyy", cid),
            arrow = debug::arrow("<->"),
            ctrl = debug::ctrl("DELSUB"),
        );
    }
}

mod publisher {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct Publisher {
        // data: Arc<PublisherData>,
    }
}

mod subscriber {
    use super::*;
    use core::ops::Deref;

    #[derive(Debug, Clone)]
    pub enum SubscriberMessage {
        Unsubscribe(Option<u64>),
        Message(Message),
    }

    impl From<Message> for SubscriberMessage {
        fn from(msg: Message) -> Self {
            Self::Message(msg)
        }
    }
    impl From<u64> for SubscriberMessage {
        fn from(max_msgs: u64) -> Self {
            Self::Unsubscribe(Some(max_msgs))
        }
    }
    impl From<Option<u64>> for SubscriberMessage {
        fn from(max_msgs: Option<u64>) -> Self {
            Self::Unsubscribe(max_msgs)
        }
    }

    #[derive(Debug, Clone)]
    pub struct SubscriberHandle {
        id: SubscriberId,
        subscriber: ActorRef<SubscriberMessage>,
    }

    impl AsRef<SubscriberId> for SubscriberHandle {
        fn as_ref(&self) -> &SubscriberId {
            &self.id
        }
    }
    impl Deref for SubscriberHandle {
        type Target = ActorRef<SubscriberMessage>;
        fn deref(&self) -> &Self::Target {
            &self.subscriber
        }
    }
    impl From<(SubscriberId, ActorRef<SubscriberMessage>)> for SubscriberHandle {
        fn from((id, subscriber): (SubscriberId, ActorRef<SubscriberMessage>)) -> Self {
            Self { id, subscriber }
        }
    }

    /// A subscriber for [`Message`]s published to subscribed topics.
    ///
    /// (Subclasses?) can be used to implement persistence, ...
    #[derive(Debug, Clone)]
    pub struct Subscriber<T> {
        pub id: SubscriberId,
        pub relay: Relay,
        pub receiver: ActorRef<T>,
    }

    impl<T: ractor::Message + From<(SubscriberId, Message)>> Subscriber<T> {
        pub fn name(&self) -> String {
            format!(
                "{prefix}-subscriber-{id}",
                prefix = &self.relay.inner.prefix,
                id = self.id
            )
        }
    }

    impl<T: ractor::Message + From<(SubscriberId, Message)>> Actor for Subscriber<T> {
        type Msg = SubscriberMessage;
        type State = Option<u64>;
        type Arguments = (Pattern, QueueGroup);

        async fn pre_start(
            &self,
            myself: ActorRef<Self::Msg>,
            (pattern, queue_group): Self::Arguments,
        ) -> Result<Self::State, ActorProcessingErr> {
            let handle = SubscriberHandle {
                id: self.id,
                subscriber: myself,
            };
            self.relay
                .add_subscriber(self.id, handle, pattern, queue_group);
            Ok(None)
        }

        async fn post_stop(
            &self,
            _myself: ActorRef<Self::Msg>,
            _state: &mut Self::State,
        ) -> Result<(), ActorProcessingErr> {
            self.relay.remove_subscriber(&self.id);
            Ok(())
        }

        async fn handle(
            &self,
            myself: ActorRef<Self::Msg>,
            msg: Self::Msg,
            state: &mut Self::State,
        ) -> Result<(), ActorProcessingErr> {
            match (msg, state.as_mut()) {
                (SubscriberMessage::Unsubscribe(max_msgs), _) => {
                    *state = max_msgs;
                }
                (SubscriberMessage::Message(msg), None) => {
                    self.receiver.cast((self.id, msg).into())?
                }
                (SubscriberMessage::Message(_), Some(0)) => myself.stop(None),
                (SubscriberMessage::Message(msg), Some(max_msgs)) => {
                    *max_msgs -= 1;
                    self.receiver.cast((self.id, msg).into())?;
                }
            }

            Ok(())
        }
    }

    // impl<T> Drop for Subscriber<T> {
    //     fn drop(&mut self) {
    //         self.relay.unsubscribe_all(self.id);
    //     }
    // }

    // impl Stream for Subscriber {
    //     type Item = ();

    //     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    //         todo!()
    //     }
    // }
}
