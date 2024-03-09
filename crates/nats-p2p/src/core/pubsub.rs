pub use self::subscriber::{Subscriber, SubscriberHandle, SubscriberId};
pub use async_nats::Subject;

use super::{debug, Message, StatusCode};
use crate::Error;
use dashmap::{mapref::multiple::RefMulti, DashMap, DashSet};
use ractor::{Actor, ActorProcessingErr, ActorRef};
use rand::{thread_rng, RngCore};
use std::sync::Arc;

type Pattern = Subject;
type SubscriptionId = (Pattern, QueueGroup);

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct QueueGroup(Option<String>);
pub type QueueGroup = Option<String>;
pub type WeightedQueueGroup = Option<(String, Option<u32>)>;

/// Shared state of all active pubsub channels (client + cluster pubs and subs).
#[derive(Debug, Clone, Default)]
pub struct Relay {
    inner: Arc<RelayState>,
}

#[derive(Debug, Default)]
struct RelayState {
    /// A map of [`SubscriberId`]s to their topics and sender.
    subscribers: DashMap<SubscriberId, SubscriberHandle>,

    /// A map of subject patterns to sets of [`SubscriberId`]s and their queue group (if any).
    subscriptions: DashMap<SubscriptionId, DashSet<SubscriberId>>,
}

impl Relay {
    pub fn publish(&self, cid: u64, message: Message) -> Result<StatusCode, Error> {
        let mut status_code = StatusCode::NO_RESPONDERS;
        // find subscribers by matching their pattern against subject
        let sub_ids = self.pick_subscribers(&message.subject);

        for sub in sub_ids.filter_map(|id| self.subscriber(id)) {
            match sub.subscriber.cast(message.clone().into()) {
                Ok(_) => {
                    status_code = StatusCode::OK;
                }
                Err(_) => {
                    // remove subscriber
                    // self.inner.subscribers.remove(&sub.pattern);
                }
            };
        }

        Ok(status_code)
    }

    /// Gets or creates a [`Subscriber`] that routes published messages to the `receiver` actor.
    pub async fn subscribe<T: ractor::Message + From<(SubscriberId, Message)>>(
        &self,
        id: SubscriberId,
        pattern: Pattern,
        queue_group: QueueGroup,
        receiver: ActorRef<T>,
    ) -> Result<(), Error> {
        // spawn (or get?) subscriber that casts to receiver
        let _handle = Subscriber::run(id, pattern, queue_group, receiver, self.clone()).await?;

        Ok(())
    }

    pub fn unsubscribe(
        &self,
        id: SubscriberId,
        max_msgs: Option<u64>,
        reason: Option<String>,
    ) -> Result<StatusCode, Error> {
        match (self.subscriber(id), max_msgs) {
            (None, _) => {}
            (Some(handle), Some(max_msgs)) => {
                handle.subscriber.cast(max_msgs.into())?;
            }
            (Some(handle), None) => {
                handle.subscriber.stop(reason);
            }
        };

        Ok(StatusCode::OK)
    }
}

impl Relay {
    fn pick_subscribers<'a>(
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
        let mut sub = self
            .inner
            .subscriptions
            .entry((pattern, queue_group))
            .or_insert_with(|| DashSet::with_capacity(3));
        sub.value_mut().insert(id);
    }

    fn remove_subscriber(&self, id: &SubscriberId) {
        let (cid, sid) = id;
        let (_, _handle) = self.inner.subscribers.remove(id).unwrap();
        self.inner.subscriptions.iter().for_each(|e| {
            e.value().remove(id);
        });

        tracing::trace!(
            "{prefix} - {arrow} [{ctrl} {sid}]",
            prefix = debug::trace_prefix("xxx.xxx.xxx.xxx:yyyy", *cid),
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

    /// (`client_id`, `sid`)
    pub type SubscriberId = (u64, u64);

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
        pub id: SubscriberId,
        pub subscriber: ActorRef<SubscriberMessage>,
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
            format!("subscriber-{}-{}", self.id.0, self.id.1)
        }

        pub async fn run(
            id: SubscriberId,
            pattern: Pattern,
            queue_group: QueueGroup,
            receiver: ActorRef<T>,
            relay: Relay,
        ) -> Result<SubscriberHandle, Error> {
            let this = Self {
                id,
                relay: relay.clone(),
                receiver,
            };
            let (subscriber, _) =
                Actor::spawn(Some(this.name()), this, (pattern, queue_group)).await?;

            Ok(SubscriberHandle { id, subscriber })
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
            match msg {
                SubscriberMessage::Unsubscribe(max_msgs) => {
                    *state = max_msgs;
                }
                SubscriberMessage::Message(msg) => match state.as_mut() {
                    None => self.receiver.cast((self.id, msg).into())?,
                    Some(0) => myself.stop(None),
                    Some(max_msgs) => {
                        *max_msgs -= 1;
                        self.receiver.cast((self.id, msg).into())?;
                    }
                },
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
