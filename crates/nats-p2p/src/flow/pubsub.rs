use super::ClientId;
use crate::{
    core::{Message, QueueGroup, Subject},
    Error,
};
use flume::{unbounded, Receiver, Sender};
use hydroflow::{
    hydroflow_syntax,
    lattices::{
        map_union::MapUnionHashMap, set_union_with_tombstones::SetUnionWithTombstonesHashSet,
        DomPair, IsBot, LatticeFrom, Max, Merge,
    },
};
use rand::{thread_rng, RngCore};
use std::{num::NonZeroU16, time::SystemTime};
use tokio::task::spawn_local;

pub use state::{ConsumerOp, Consumers, SubscriberId};
mod state {
    use super::*;

    type VClock = MapUnionHashMap<u64, Max<usize>>;
    type Anna<K, V> = MapUnionHashMap<K, DomPair<VClock, V>>;

    // type SubscriptionId = (ClientId, u64);
    // type Subscription = (SubscriptionId, Subject);

    pub type StreamId = (Subject, QueueGroup);

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub enum SubscriberId {
        Client(ClientId, u64),
        Temp(ClientId, u64),
    }
    impl From<(ClientId, u64)> for SubscriberId {
        fn from((id, sid): (ClientId, u64)) -> Self {
            Self::Client(id, sid)
        }
    }
    impl From<SubscriberId> for ClientId {
        fn from(sub: SubscriberId) -> Self {
            match sub {
                SubscriberId::Client(id, _) => id,
                SubscriberId::Temp(id, _) => id,
            }
        }
    }

    // pub type ConsumerOp = PersistenceKeyed<(StreamId, SubscriberId), ()>;
    pub type ConsumerSet = SetUnionWithTombstonesHashSet<SubscriberId>;

    pub enum ConsumerOp {
        Create(StreamId, SubscriberId),
        Remove(SubscriberId),
    }

    #[derive(Debug, Clone, Default)]
    pub struct Consumers {
        pub inner: MapUnionHashMap<StreamId, ConsumerSet>,
        // inner: Anna<StreamId, SetUnionHashSet<SubscriberId>>
    }

    impl Consumers {
        pub fn into_queues(self) -> impl Iterator<Item = (Subject, Vec<SubscriberId>)> {
            self.inner
                .into_reveal()
                .into_iter()
                .map(|((topic, queue), subs)| {
                    let (pos, neg) = subs.as_reveal_ref();
                    let ids = pos.difference(neg).copied().collect::<Vec<_>>();

                    if queue.is_some() {
                        let idx = thread_rng().next_u32() as usize % ids.len();
                        (topic, vec![ids[idx]])
                    } else {
                        (topic, ids)
                    }
                })
        }
    }

    impl IsBot for Consumers {
        fn is_bot(&self) -> bool {
            self.inner.is_bot()
        }
    }
    impl LatticeFrom<Consumers> for Consumers {
        fn lattice_from(other: Consumers) -> Self {
            other
        }
    }
    impl Merge<Consumers> for Consumers {
        fn merge(&mut self, other: Consumers) -> bool {
            self.inner.merge(other.inner)
        }
    }
    impl Merge<ConsumerOp> for Consumers {
        fn merge(&mut self, op: ConsumerOp) -> bool {
            match op {
                ConsumerOp::Create(sub_id, sub) => {
                    let subs = ConsumerSet::new_from([sub], []);
                    self.inner.as_reveal_mut().insert(sub_id, subs);
                    true
                }
                ConsumerOp::Remove(sub) => {
                    for (_, subs) in self.inner.as_reveal_mut().iter_mut() {
                        if subs.as_reveal_ref().0.contains(&sub) {
                            subs.as_reveal_mut().1.insert(sub);
                        }
                    }
                    true
                }
            }
        }
    }
}

pub type Commander = Sender<(ClientId, Command)>;
pub type Responder = Receiver<(ClientId, Message)>;

#[derive(Debug, Clone)]
pub enum Command {
    Pub(Message),
    Sub {
        subject: Subject,
        queue_group: QueueGroup,
        sid: u64,
    },
    Unsub {
        sid: u64,
        max_msgs: Option<u64>,
    },
}

pub async fn run(num_threads: Option<NonZeroU16>) -> Result<(Commander, Responder), Error> {
    let (ingress_send, ingress_recv) = unbounded();
    let (egress_send, egress_recv) = unbounded();

    let sub_id = 1u64;
    let mut hf = hydroflow_syntax! {
        /******************** state ********************/
        /*
         * (subject, queue_group) -> {(client_id, sid)}
         */
        consumers = union()
            -> identity::<ConsumerOp>()
            -> fold::<'static>(Consumers::default, |subs, op| { subs.merge(op); })
            -> tee();

        /*
         * subject -> {(client_id, sid)}
         */
        subscribers = consumers[queues]
            -> flat_map(Consumers::into_queues);

        /******************** commands ********************/
        cmds = source_stream(ingress_recv.into_stream())
            -> demux(|v, var_args!(Pub, Sub, Unsub)| match v {
                (id, Command::Pub(msg)) => Pub.give((id, msg)),
                (id, Command::Sub { subject, queue_group, sid }) => Sub.give((id, sid, (subject, queue_group))),
                (id, Command::Unsub { sid, max_msgs }) => Unsub.give((id, sid, max_msgs)),
            });

        /*
         * publish
         */
        pubs = cmds[Pub] -> tee();

        // route published messages to subscribers
        pubs[publish] -> [1]deliveries;
        subscribers -> [0]deliveries;
        deliveries = cross_join::<'tick, 'tick>()
            -> filter(|((topic, _ids), (_, msg))| topic.matches(&msg.subject))
            -> tee();
        deliveries[responses]
            -> flat_map(|((_, ids), (_, msg))|
                ids.into_iter().map(move |id| (id, msg.clone())))
            -> responses;

        /*
         * subscribe
         */
        subs = cmds[Sub] -> tee();
        subs[create]
            -> map(|(id, sid, sub_id)| ConsumerOp::Create(sub_id, (id, sid).into()))
            -> consumers[create];

        /*
         * unsubscribe
         */
        unsubs = cmds[Unsub] -> demux(|v, var_args!(Now, Soon)| match v {
            (id, sid, None) => Now.give((id, sid)),
            (id, sid, Some(max_msgs)) => Soon.give((id, sid, max_msgs)),
        });
        unsubs[Now]
            -> map(|(id, sid)| ConsumerOp::Remove((id, sid).into()))
            -> consumers[remove];
        unsubs[Soon]
            // -> map(|((cid, _), sid, max_msgs)| ConsumerOp::Remove((cid, sid).into()))
            -> null();

        /******************** responses ********************/
        responses = union()
            -> identity::<(SubscriberId, Message)>()
            -> map(|(id, msg)| (id.into(), msg))
            -> dest_sink(egress_send.into_sink());
    };

    spawn_local(async move {
        hf.run_async().await;
    });

    Ok((ingress_send, egress_recv))
}

mod jetstream2 {
    use super::*;
    use async_nats::jetstream::{
        consumer::{
            AckPolicy, Config as ConsumerConfig, DeliverPolicy, Info as ConsumerInfo, ReplayPolicy,
            SequenceInfo,
        },
        stream::Info as StreamInfo,
    };

    /// Jetstream stream.
    pub struct Stream {
        /// The stream's configuration and state.
        info: StreamInfo,
    }

    pub struct Consumer {
        /// The consumer's configuration and state.
        info: ConsumerInfo,
    }

    impl Consumer {
        pub fn new_basic(subject: Subject, queue_group: QueueGroup, sid: u64) -> Self {
            let config = ConsumerConfig {
                name: queue_group,
                // emulate basic, push-based subscriber
                deliver_subject: Some(subject.to_string()),
                durable_name: None,
                deliver_policy: DeliverPolicy::New,
                ack_policy: AckPolicy::None,
                max_deliver: 0,
                replay_policy: ReplayPolicy::Instant,
                headers_only: false,
                ..Default::default()
            };

            let info = ConsumerInfo {
                config,
                stream_name: Default::default(),
                name: Default::default(),
                created: SystemTime::now().into(),
                delivered: SequenceInfo {
                    consumer_sequence: 0,
                    stream_sequence: 0,
                    last_active: None,
                },
                ack_floor: SequenceInfo {
                    consumer_sequence: 0,
                    stream_sequence: 0,
                    last_active: None,
                },
                num_ack_pending: 0,
                num_redelivered: 0,
                num_waiting: 0,
                num_pending: 0,
                cluster: None,     // TODO
                push_bound: false, // TODO
            };

            Self { info }
        }
    }
}
