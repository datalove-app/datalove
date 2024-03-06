use crate::{Error, Subject};
use async_nats::Message;
use dashmap::DashMap;
use flume;
use futures::Stream;
use ractor::{ActorRef, OutputPort};
use std::{
    collections::HashMap,
    collections::HashSet,
    pin::Pin,
    sync::{atomic::AtomicU64, Arc, OnceLock, RwLock},
    task::{Context, Poll},
};

static PUBLISH: OnceLock<Arc<OutputPort<Message>>> = OnceLock::new();
fn publish_port() -> &'static Arc<OutputPort<Message>> {
    PUBLISH.get_or_init(|| Arc::new(OutputPort::default()))
}

/// A map of subjects to their topic ID.
type TopicMap = DashMap<Subject, u64>;
///
type PublisherMap = DashMap<u64, publisher::Publisher>;
/// A map of subscriber IDs to their topics and sender.
type SubscriberMap = DashMap<u64, subscriber::SubscriberInfo>;
/// A map of subjects to a map of ...
type SubscriptionMap = DashMap<Subject, HashSet<u64>>;

/// Shared state of all active pubsub channels (client + cluster pubs and subs).
#[derive(Clone, Debug, Default)]
pub struct Relay {
    inner: Arc<RelayState>,
}

#[derive(Debug, Default)]
struct RelayState {
    topics: TopicMap,
    publishers: PublisherMap,
    subscribers: SubscriberMap,
    subscriptions: SubscriptionMap,
    last_topic_id: AtomicU64,
    last_subscriber_id: AtomicU64,
}

impl Relay {
    pub async fn get_publisher(
        &self,
        publisher: ActorRef<Message>,
    ) -> Result<publisher::Publisher, Error> {
        todo!()
    }

    pub async fn get_subscriber(&self) -> Result<subscriber::Subscriber, Error> {
        // let id = self
        //     .data
        //     .last_subscriber_id
        //     .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        // let (sender, receiver) = flume::unbounded();
        // let relay = self.clone();
        // let data = Arc::new(SubscriberData {
        //     id,
        //     relay,
        //     receiver,
        // });

        // store sender

        // Subscriber { data }
        todo!()
    }

    /// Routes a message from a publisher to any and all subscribers of a topic.
    pub async fn route(&self) -> Result<(), Error> {
        todo!()
    }

    // pub async fn publish(&self, topic: &str, message: &[u8]) -> Result<(), Error> {
    //     todo!()
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

    /// A subscriber for [`Message`]s published to subscribed topics.
    #[derive(Debug, Clone)]
    #[must_use]
    pub struct Subscriber {
        data: Arc<SubscriberData>,
    }

    impl Stream for Subscriber {
        type Item = ();

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            todo!()
        }
    }

    #[derive(Debug, Clone)]
    pub struct SubscriberData {
        id: u64,
        relay: Relay,
        receiver: flume::Receiver<()>,
    }

    // impl Drop for SubscriberData {
    //     fn drop(&mut self) {
    //         self.relay.unsubscribe_all(self.id);
    //     }
    // }

    #[derive(Debug, Clone)]
    pub struct SubscriberInfo {
        sender: flume::Sender<()>,
        topics: HashSet<u64>,
    }
}
