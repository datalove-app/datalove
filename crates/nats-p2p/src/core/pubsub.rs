pub use self::subscriber::{Subscriber, SubscriberHandle, SubscriberId};
pub use async_nats::Subject;

use super::{Message, StatusCode};
use crate::Error;
use dashmap::{mapref::multiple::RefMulti, DashMap, DashSet};
use ractor::{pg, Actor, ActorProcessingErr, ActorRef, OutputPort};
use rand::{thread_rng, RngCore};
use std::{collections::HashSet, sync::Arc};

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
    // /// A map of subject patterns to their topic ID.
    // patterns: DashMap<Pattern, u64>,
    // queue_groups: DashMap<String, DashSet<SubscriberId>>,

    // ///
    // publishers: DashMap<u64, Publisher>,
    /// A map of [`SubscriberId`]s to their topics and sender.
    subscribers: DashMap<SubscriberId, SubscriberHandle>,

    /// A map of subject patterns to sets of [`SubscriberId`]s and their queue group (if any).
    subscriptions: DashMap<SubscriptionId, DashSet<SubscriberId>>,
    // /// The last subscription ID used.
    // last_subscription_id: AtomicU64,
}

impl Relay {
    // /// Gets or spawns a [`Publisher`] actor that subscribes to the
    // /// `publisher`'s [`OutputPort`].
    // pub async fn get_publisher<T>(&self, publisher: ActorRef<T>) -> Result<Publisher, Error> {
    //     todo!()
    // }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn publish(&self, client_id: u64, message: Message) -> Result<StatusCode, Error> {
        let mut status_code = StatusCode::NO_RESPONDERS;

        // find subscribers by matching their pattern against subject
        for sub in self
            .pick_subscribers(&message.subject)
            .filter_map(|id| self.subscriber(id))
        {
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
    #[tracing::instrument(level = "trace", skip(self, receiver))]
    pub async fn subscribe<T: ractor::Message + From<(SubscriberId, Message)>>(
        &self,
        id: SubscriberId,
        pattern: Pattern,
        queue_group: QueueGroup,
        receiver: ActorRef<T>,
    ) -> Result<SubscriberHandle, Error> {
        // get or spawn subscriber that casts to receiver
        if let Some(subscriber) = self.subscriber(id) {
            return Ok(subscriber.clone());
        }

        let handle = Subscriber::spawn(id, receiver, self.clone()).await?;

        // store subscriber in relay by its pattern
        self.add_subscriber(id, handle.clone(), pattern, queue_group);

        Ok(handle)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn unsubscribe(
        &self,
        id: SubscriberId,
        max_msgs: Option<u64>,
        reason: Option<String>,
    ) -> Result<StatusCode, Error> {
        match (self.subscriber(id), max_msgs) {
            (None, _) => {}
            (Some(subscriber), Some(max_msgs)) => {
                subscriber.subscriber.cast(max_msgs.into())?;
            }
            (Some(subscriber), None) => {
                subscriber.subscriber.stop(reason);
            }
        };

        Ok(StatusCode::OK)
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
            .or_insert_with(|| {
                let dash_set = DashSet::with_capacity(3);
                dash_set.insert(id);
                dash_set
            });
    }

    fn remove_subscriber(&self, id: &SubscriberId) {
        self.inner.subscribers.remove(id);
        self.inner.subscriptions.iter().for_each(|e| {
            e.value().remove(id);
        });
        tracing::trace!(
            "{} - cid:{} - <-> [DELSUB {}]",
            "xxx.xxx.xxx.xxx:yyyy",
            id.0,
            id.1
        );
    }

    fn subscriber(&self, id: SubscriberId) -> Option<SubscriberHandle> {
        self.inner.subscribers.get(&id).map(|v| v.clone())
    }

    fn pick_queue_group_subscriber(&self, set: &DashSet<SubscriberId>) -> Option<SubscriberHandle> {
        let len = set.len();
        if len == 0 {
            return None;
        }

        let set = set.iter().map(|id| *id).collect::<Vec<_>>();
        tracing::info!("set: {:?}", set);

        let roll = || {
            let id = set[rand::thread_rng().next_u32() as usize % len];
            self.subscriber(id)
        };

        for _ in 0..len {
            if let Some(subscriber) = roll() {
                return Some(subscriber);
            }
        }

        None
    }

    fn pick_subscribers<'a>(
        &'a self,
        subject: &'a Subject,
    ) -> impl Iterator<Item = SubscriberId> + 'a {
        enum Iter<I> {
            Single(Option<SubscriberId>),
            Multi(I),
        }

        impl<I: Iterator<Item = SubscriberId>> Iterator for Iter<I> {
            type Item = SubscriberId;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Self::Single(subscriber) => subscriber.take(),
                    Self::Multi(iter) => iter.next(),
                }
            }
        }

        self.filter_subscriptions(subject)
            .flat_map(move |e| match e.key().1 {
                Some(_) => {
                    let set = e.value().iter().map(|id| *id).collect::<Vec<_>>();
                    let idx = thread_rng().next_u32() as usize % set.len();
                    Iter::Single(set.get(idx).copied())
                }
                None => Iter::Multi(
                    e.value()
                        .iter()
                        .map(|id| *id)
                        .collect::<Vec<_>>()
                        .into_iter(),
                ),
            })
    }

    fn filter_subscriptions<'a>(
        &'a self,
        subject: &'a Subject,
    ) -> impl Iterator<Item = RefMulti<'a, SubscriptionId, DashSet<SubscriberId>>> + 'a {
        self.inner
            .subscriptions
            .iter()
            .filter(move |e| subject.matches(&e.key().0) && !e.value().is_empty())
    }

    // fn filter_subscriber_ids(&self, subject: &Subject) -> HashSet<SubscriberId> {
    //     self.inner
    //         .subscriptions
    //         .iter()
    //         .filter(|e| subject.matches(e.key()))
    //         .flat_map(|e| e.value().iter().map(|e| *e).collect::<HashSet<_>>())
    //         .collect()
    // }
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

        pub async fn spawn(
            id: SubscriberId,
            receiver: ActorRef<T>,
            relay: Relay,
        ) -> Result<SubscriberHandle, Error> {
            let this = Self {
                id,
                relay,
                receiver,
            };
            let (subscriber, _) = Actor::spawn(Some(this.name()), this, ()).await?;

            Ok(SubscriberHandle { id, subscriber })
        }
    }

    impl<T: ractor::Message + From<(SubscriberId, Message)>> Actor for Subscriber<T> {
        type Msg = SubscriberMessage;
        type State = Option<u64>;
        type Arguments = ();

        async fn pre_start(
            &self,
            _myself: ActorRef<Self::Msg>,
            _args: Self::Arguments,
        ) -> Result<Self::State, ActorProcessingErr> {
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
