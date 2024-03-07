pub use self::subscriber::{Subscriber, SubscriberId, SubscriberInfo};
pub use async_nats::Subject;

use super::{Message, StatusCode};
use crate::Error;
use dashmap::{DashMap, DashSet};
use futures::Stream;
use ractor::{pg, Actor, ActorProcessingErr, ActorRef, OutputPort};
use std::{
    collections::HashSet,
    pin::Pin,
    sync::{atomic::AtomicU64, Arc, OnceLock, RwLock},
    task::{Context, Poll},
};

// static PUBLISH: OnceLock<Arc<OutputPort<Message>>> = OnceLock::new();
// fn publish_port() -> &'static Arc<OutputPort<Message>> {
//     PUBLISH.get_or_init(|| Arc::new(OutputPort::default()))
// }

type Pattern = Subject;

/// Shared state of all active pubsub channels (client + cluster pubs and subs).
#[derive(Debug, Clone, Default)]
pub struct Relay {
    inner: Arc<RelayState>,
}

#[derive(Debug, Default)]
struct RelayState {
    // /// A map of subject patterns to their topic ID.
    // patterns: DashMap<Pattern, u64>,

    // ///
    // publishers: DashMap<u64, Publisher>,
    /// A map of [`SubscriberId`]s to their topics and sender.
    subscribers: DashMap<SubscriberId, SubscriberInfo>,

    /// A map of subject patterns to sets of [`SubscriberId`]s.
    subscriptions: DashMap<Pattern, DashSet<SubscriberId>>,
    // /// The last topic ID used.
    // last_topic_id: AtomicU64,
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
        for sub in self.filter_subscribers(&message.subject).into_iter() {
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
        queue_group: Option<String>,
        receiver: ActorRef<T>,
    ) -> Result<SubscriberInfo, Error> {
        // spawn subscriber that casts to receiver
        // store subscriber in relay by its pattern

        if let Some(subscriber) = self.subscriber(id) {
            return Ok(subscriber);
        }

        let info = {
            let (subscriber, _) = Actor::spawn(
                Some(format!("subscriber-{}-{}", id.0, id.1)),
                Subscriber {
                    id,
                    relay: self.clone(),
                    receiver,
                },
                (),
            )
            .await?;

            if let Some(queue_group) = queue_group {
                // pg::join_scoped(pattern.to_string(), queue_group, vec![subscriber.clone()]);
            }

            SubscriberInfo { id, subscriber }
        };

        self.inner.subscribers.insert(id, info.clone());
        self.inner
            .subscriptions
            .entry(pattern)
            .or_default()
            .insert(id);

        Ok(info)
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

    fn subscriber(&self, id: SubscriberId) -> Option<SubscriberInfo> {
        self.inner.subscribers.view(&id, |_, info| info.clone())
    }

    fn filter_subscribers(&self, subject: &Subject) -> impl Iterator<Item = SubscriberInfo> + '_ {
        self.filter_subscriber_ids(subject)
            .into_iter()
            .filter_map(|id| self.subscriber(id))
    }

    fn filter_subscriber_ids(&self, subject: &Subject) -> HashSet<SubscriberId> {
        self.inner
            .subscriptions
            .iter()
            .filter(|e| subject.matches(e.key()))
            .flat_map(|e| e.value().iter().map(|e| *e).collect::<HashSet<_>>())
            .collect()
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
    pub struct SubscriberInfo {
        pub id: SubscriberId,
        pub subscriber: ActorRef<SubscriberMessage>,
    }

    /// A subscriber for [`Message`]s published to subscribed topics.
    #[derive(Debug, Clone)]
    pub struct Subscriber<T> {
        pub id: SubscriberId,
        pub relay: Relay,
        pub receiver: ActorRef<T>,
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
            tracing::trace!(
                "{} - cid:{} - <-> [DELSUB {}]",
                "todo",
                self.id.0,
                self.id.1
            );

            self.relay.inner.subscribers.remove(&self.id);
            self.relay.inner.subscriptions.iter().for_each(|e| {
                e.value().remove(&self.id);
            });
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
