use super::{codec::Codec, ConnectInfo, ServerInfo};
use crate::{Error, Server, Subject};
use async_nats::{rustls::server, HeaderMap, StatusCode};
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use ractor::{
    cast, time::send_interval, Actor, ActorProcessingErr, ActorRef, RactorErr, SupervisionEvent,
};
use std::{fmt, marker::PhantomData, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::RwLock,
    task::JoinHandle,
};
use tokio_util::codec::{FramedRead, FramedWrite};

/// `ClientOp` represents all actions of `Client`.
///
/// [Original documentation](https://docs.nats.io/reference/reference-protocols/nats-protocol)
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClientOp {
    /// `CONNECT {["option_name":option_value],...}`
    Connect(ConnectInfo),

    /// `INFO {["option_name":option_value],...}`
    Info(Option<ServerInfo>),

    /// `PING`
    Ping,

    /// `PONG`
    Pong,

    /// `PUB <subject> [reply-to] <#bytes>\r\n[payload]`
    /// `HPUB <subject> [reply-to] <#header bytes> <#total bytes>\r\n[headers]\r\n\r\n[payload]`
    Publish {
        subject: Subject,
        reply_to: Option<Subject>,
        headers: Option<HeaderMap>,
        payload: Bytes,
    },

    /// `SUB <subject> [queue group] <sid>`
    Subscribe {
        sid: u64,
        subject: Subject,
        queue_group: Option<String>,
    },

    /// `UNSUB <sid> [max_msgs]`
    Unsubscribe { sid: u64, max_msgs: Option<u64> },
}

impl ClientOp {
    const fn control(&self) -> &'static str {
        match self {
            Self::Connect(_) => "CONNECT",
            Self::Info(_) => "INFO",
            Self::Ping => "PING",
            Self::Pong => "PONG",
            Self::Publish { headers: None, .. } => "PUB",
            Self::Publish { .. } => "HPUB",
            Self::Subscribe { .. } => "SUB",
            Self::Unsubscribe { .. } => "UNSUB",
        }
    }
}

impl fmt::Display for ClientOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientOp::Connect(_) => write!(f, "CONNECT ..."),
            ClientOp::Info(_) => write!(f, "INFO ..."),
            ClientOp::Ping => write!(f, "PING"),
            ClientOp::Pong => write!(f, "PONG"),
            ClientOp::Publish {
                subject, reply_to, ..
            } => {
                write!(
                    f,
                    "PUB {}{}",
                    subject,
                    reply_to.as_ref().map_or("".into(), |r| format!(" {}", r))
                )
            }
            ClientOp::Subscribe {
                sid,
                subject,
                queue_group,
            } => {
                write!(
                    f,
                    "SUB {}{} {}",
                    subject,
                    queue_group
                        .as_ref()
                        .map_or("".into(), |q| format!(" {}", q)),
                    sid
                )
            }
            ClientOp::Unsubscribe { sid, max_msgs } => {
                write!(
                    f,
                    "UNSUB {}{}",
                    sid,
                    max_msgs.as_ref().map_or("".into(), |m| format!(" {}", m))
                )
            }
        }
    }
}

/// [Original core documentation](https://docs.nats.io/reference/reference-protocols/nats-protocol)
/// [Original cluster documentation](https://docs.nats.io/reference/reference-protocols/nats-server-protocol)
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServerOp {
    /// `CONNECT {["option_name":option_value],...}`
    Connect(ConnectInfo),

    /// `INFO {["option_name":option_value],...}`
    Info(ServerInfo),

    /// `PING`
    Ping,

    /// `PONG`
    Pong,

    /// +OK
    Ok,

    /// `-ERR <error message>`
    Err(String),

    /// `RS+ <account> <subject> [queue-group] [weight]
    Subscribe {
        account: String,
        subject: Subject,
        queue_group: Option<(String, Option<u32>)>,
    },

    /// `RS- <account> <subject>`
    Unsubscribe { account: String, subject: Subject },

    /// `MSG <subject> <sid> [reply-to] <#bytes>\r\n[payload]`
    /// `HMSG <subject> <sid> [reply-to] <#header-bytes> <#total-bytes>\r\n<version line>\r\n[headers]\r\n\r\n[payload]`
    /// `RMSG <account> <subject> [reply-to] <#bytes>\r\n[payload]`
    Message {
        sid: u64,
        account: Option<String>,
        subject: Subject,
        reply_to: Option<Subject>,
        headers: Option<HeaderMap>,
        status: Option<StatusCode>,
        description: Option<String>,
        payload: Bytes,
        // length: usize,
    },
}

impl ServerOp {
    const fn control(&self) -> &'static str {
        match self {
            Self::Connect(_) => "CONNECT",
            Self::Info(_) => "INFO",
            Self::Ping => "PING",
            Self::Pong => "PONG",
            Self::Ok => "+OK",
            Self::Err(_) => "-ERR",
            Self::Subscribe { .. } => "RS+",
            Self::Unsubscribe { .. } => "RS-",
            Self::Message { account, .. } if account.is_some() => "RMSG",
            Self::Message { headers: None, .. } => "MSG",
            Self::Message { .. } => "HMSG",
        }
    }
}

impl fmt::Display for ServerOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ctrl = self.control();
        match self {
            ServerOp::Connect(_) => write!(f, "{ctrl} ..."),
            ServerOp::Info(_) => write!(f, "{ctrl} ..."),
            ServerOp::Ok | ServerOp::Ping | ServerOp::Pong => f.write_str(ctrl),
            ServerOp::Err(err) => write!(f, "{ctrl} {}", err),
            ServerOp::Subscribe {
                account,
                subject,
                queue_group,
            } => {
                write!(
                    f,
                    "{ctrl} {account} {subject} {}",
                    queue_group
                        .as_ref()
                        .map_or("".into(), |(q, w)| format!(" {} {:?}", q, w))
                )
            }
            ServerOp::Unsubscribe { account, subject } => {
                write!(f, "{ctrl} {account} {subject}")
            }
            ServerOp::Message {
                sid,
                account,
                subject,
                reply_to,
                // headers,
                status,
                description,
                // payload,
                ..
            } => {
                write!(
                    f,
                    "{ctrl} {subject} {}{}{}{}...",
                    account
                        .as_ref()
                        .map_or(format!(" {}", sid), |a| format!(" {}", a)),
                    reply_to.as_ref().map_or("".into(), |r| format!(" {}", r)),
                    status.as_ref().map_or("".into(), |s| format!(" {}", s)),
                    description
                        .as_ref()
                        .map_or("".into(), |d| format!(" {}", d)),
                    // headers,
                    // payload.len(),
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum CoreMessage {
    Incoming(ClientOp),
    Outgoing(ServerOp),
}

/*
 * Actors
 */

#[derive(Debug)]
pub struct ClientSession<T> {
    _io: PhantomData<T>,
}

impl<T> ClientSession<T> {
    pub fn name(id: u64) -> String {
        format!("client-session-{}", id)
    }
    pub fn recv_name(id: u64) -> String {
        format!("client-session-{}-recv", id)
    }
    pub fn send_name(id: u64) -> String {
        format!("client-session-{}-send", id)
    }
}

impl<T> Default for ClientSession<T> {
    fn default() -> Self {
        Self { _io: PhantomData }
    }
}

#[derive(Debug)]
pub struct ClientSessionArgs<T> {
    pub io: T,
    pub inbox_prefix: Option<String>,
    pub server_info: ServerInfo,
}

#[derive(Debug)]
pub struct ClientState {
    receiver: ActorRef<ClientOp>,
    sender: ActorRef<ServerOp>,
    server_info_timer: Option<JoinHandle<()>>,
    // shutdown: Option<JoinHandle<()>>,
    data: Arc<ClientData>,
    // peer_addr: SocketAddr,
    // host_addr: SocketAddr,
}

impl ClientState {
    fn set_connect_info(&mut self, connect_info: ConnectInfo) {
        Arc::get_mut(&mut self.data).map(|d| d.connect_info.replace(connect_info));
    }

    fn set_server_info(&mut self, server_info: ServerInfo) {
        if let Some(ref mut timer) = self.server_info_timer {
            timer.abort();
        }
        Arc::get_mut(&mut self.data).map(|d| d.server_info = server_info.clone());
        self.server_info_timer =
            Some(self.sender.send_interval(Duration::from_secs(60), move || {
                ServerOp::Info(server_info.clone())
            }));
    }
}

#[derive(Debug)]
pub struct ClientData {
    session: ActorRef<CoreMessage>,
    inbox_prefix: String,
    connect_info: Option<ConnectInfo>,
    request_timeout: Option<Duration>,
    server_info: ServerInfo,
}

impl ClientData {
    fn client_id(&self) -> u64 {
        self.server_info.client_id
    }
    fn verbose(&self) -> bool {
        self.connect_info.as_ref().map_or(false, |c| c.verbose)
    }

    fn trace_incoming(&self, op: &ClientOp) {
        tracing::trace!(
            "{} - cid:{} - <<- [{}]",
            self.server_info.client_ip,
            self.server_info.client_id,
            op
        );
    }

    fn trace_outgoing(&self, op: &ServerOp) {
        tracing::trace!(
            "{} - cid:{} - ->> [{}]",
            self.server_info.client_ip,
            self.server_info.client_id,
            op
        );
    }
}

impl<T> Actor for ClientSession<T>
where
    T: AsyncRead + AsyncWrite + Send + Sync + Unpin + 'static,
{
    type Msg = CoreMessage;
    type State = ClientState;
    type Arguments = ClientSessionArgs<T>;

    /// Spawns sender and receiver actors from [`Framed`] IO parts.
    ///
    /// [`Framed`]: tokio_util::codec::Framed
    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        ClientSessionArgs {
            io,
            inbox_prefix,
            server_info,
        }: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let data = Arc::new(ClientData {
            session: myself.clone(),
            inbox_prefix: inbox_prefix.unwrap_or_else(|| "_INBOX".into()),
            connect_info: None,
            request_timeout: None,
            server_info: server_info.clone(),
        });

        let (receiver, sender) = {
            let (reader, writer) = tokio::io::split(io);
            let stream = FramedRead::new(reader, Codec::<ClientOp>::default());
            let sink = FramedWrite::new(writer, Codec::<ServerOp>::default());

            let (sender, _) = Actor::spawn_linked(
                Some(Self::send_name(data.client_id())),
                sender::ClientSender::new(),
                (data.clone(), sink),
                myself.get_cell(),
            )
            .await?;

            let (receiver, _) = Actor::spawn_linked(
                Some(Self::recv_name(data.client_id())),
                receiver::ClientReceiver::new(),
                (data.clone(), stream),
                myself.get_cell(),
            )
            .await?;

            (receiver, sender)
        };

        let mut state = ClientState {
            receiver,
            sender,
            server_info_timer: None,
            data,
        };
        state.set_server_info(server_info.clone());
        Ok(state)
    }

    /// Sends initial [`ServerInfo`] and starts session shutdown task.
    async fn post_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        state
            .sender
            .cast(ServerOp::Info(state.data.server_info.clone()))?;

        // todo: shutdown timer

        Ok(())
    }

    async fn post_stop(
        &self,
        _myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        tracing::info!(
            "Client session closed for {}",
            state.data.server_info.client_ip
        );
        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            // todo: reset shutdown timer
            CoreMessage::Incoming(op) => match &op {
                // update client connect info
                ClientOp::Connect(connect_info) => {
                    state.set_connect_info(connect_info.clone());
                }
                _ => state.receiver.cast(op)?,
            },
            CoreMessage::Outgoing(op) => match &op {
                // update server info, (re)start timer
                ServerOp::Info(server_info) => {
                    state.set_server_info(server_info.clone());
                }
                _ => state.sender.cast(op)?,
            },
        }

        Ok(())
    }

    async fn handle_supervisor_evt(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: SupervisionEvent,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            SupervisionEvent::ActorPanicked(actor, msg) => {
                if actor.get_id() == state.receiver.get_id() {
                    Error::server("ClientReceiver panicked", anyhow::anyhow!(msg));
                } else if actor.get_id() == state.sender.get_id() {
                    Error::server("ClientSender panicked", anyhow::anyhow!(msg));
                } else {
                    Error::server("Unknown actor panicked", anyhow::anyhow!(msg));
                };

                myself.stop(Some("child panic".to_string()))
            }
            SupervisionEvent::ActorTerminated(actor, _, reason) => {
                if actor.get_id() == state.receiver.get_id() {
                    Error::server(
                        "ClientReceiver terminated",
                        anyhow::anyhow!(reason.unwrap_or_default()),
                    );
                } else if actor.get_id() == state.sender.get_id() {
                    Error::server(
                        "ClientSender terminated",
                        anyhow::anyhow!(reason.unwrap_or_default()),
                    );
                } else {
                    Error::server(
                        "Unknown actor terminated",
                        anyhow::anyhow!(reason.unwrap_or_default()),
                    );
                };

                myself.stop(Some("child terminated".to_string()))
            }
            _ => {
                // tracing::warn!("Unhandled supervisor event: {:?}", msg);
            }
        };

        Ok(())
    }
}

mod sender {
    use super::*;

    #[derive(Debug)]
    pub struct ClientSender<S> {
        _sink: PhantomData<S>,
    }

    impl<S> ClientSender<S> {
        pub fn new() -> Self {
            Self { _sink: PhantomData }
        }
    }

    type ClientSenderArgs<S> = (Arc<ClientData>, FramedWrite<S, Codec<ServerOp>>);

    pub struct ClientSenderState<S> {
        framed_write: Option<FramedWrite<S, Codec<ServerOp>>>,
        data: Arc<ClientData>,
    }

    impl<S> Actor for ClientSender<S>
    where
        S: AsyncWrite + Send + Sync + Unpin + 'static,
    {
        type Msg = ServerOp;
        type State = ClientSenderState<S>;
        type Arguments = ClientSenderArgs<S>;

        async fn pre_start(
            &self,
            _myself: ActorRef<Self::Msg>,
            (data, framed_write): Self::Arguments,
        ) -> Result<Self::State, ActorProcessingErr> {
            Ok(ClientSenderState {
                data,
                framed_write: Some(framed_write),
            })
        }

        async fn post_stop(
            &self,
            _myself: ActorRef<Self::Msg>,
            state: &mut Self::State,
        ) -> Result<(), ActorProcessingErr> {
            drop(state.framed_write.take());
            Ok(())
        }

        async fn handle(
            &self,
            _myself: ActorRef<Self::Msg>,
            msg: Self::Msg,
            state: &mut Self::State,
        ) -> Result<(), ActorProcessingErr> {
            state.data.trace_outgoing(&msg);

            state
                .framed_write
                .as_mut()
                .expect("should not handle messages after dropping framed write")
                .send(msg)
                .await?;

            Ok(())
        }
    }
}

mod receiver {
    use futures::TryStreamExt;

    use super::*;

    #[derive(Debug)]
    pub struct ClientReceiver<S>(PhantomData<S>);

    impl<S> ClientReceiver<S> {
        pub fn new() -> Self {
            Self(PhantomData)
        }
    }

    type ClientReceiverArgs<S> = (Arc<ClientData>, FramedRead<S, Codec<ClientOp>>);

    pub struct ClientReceiverState {
        framed_read: Option<JoinHandle<Result<(), Error>>>,
        data: Arc<ClientData>,
    }

    impl<S> Actor for ClientReceiver<S>
    where
        S: AsyncRead + Send + Sync + Unpin + 'static,
    {
        type Msg = ClientOp;
        type State = ClientReceiverState;
        type Arguments = ClientReceiverArgs<S>;

        async fn pre_start(
            &self,
            myself: ActorRef<Self::Msg>,
            (data, mut stream): Self::Arguments,
        ) -> Result<Self::State, ActorProcessingErr> {
            // map read stream to messages
            // send connect msgs to session to be cached
            let session = data.session.clone();
            let framed_read = tokio::spawn(async move {
                while let Some(op) = stream.try_next().await? {
                    if let ClientOp::Connect(_) = &op {
                        session.cast(CoreMessage::Incoming(op))?;
                    } else {
                        myself.cast(op)?;
                    }
                }

                Ok::<(), Error>(())
            });

            Ok(ClientReceiverState {
                data,
                framed_read: Some(framed_read),
            })
        }

        async fn post_stop(
            &self,
            _myself: ActorRef<Self::Msg>,
            state: &mut Self::State,
        ) -> Result<(), ActorProcessingErr> {
            drop(state.framed_read.take());
            Ok(())
        }

        async fn handle(
            &self,
            _myself: ActorRef<Self::Msg>,
            msg: Self::Msg,
            state: &mut Self::State,
        ) -> Result<(), ActorProcessingErr> {
            state.data.trace_incoming(&msg);

            let sender = ActorRef::<ServerOp>::where_is(ClientSession::<()>::send_name(
                state.data.client_id(),
            ))
            .ok_or_else(|| Error::actor("client sender should exist"))?;

            match msg {
                // route quick messages directly to sender
                ClientOp::Connect(_) => {
                    if state.data.verbose() {
                        sender.cast(ServerOp::Ok)?;
                    }
                }
                ClientOp::Info(_) => sender.cast(ServerOp::Info(state.data.server_info.clone()))?,
                ClientOp::Ping => sender.cast(ServerOp::Pong)?,
                ClientOp::Pong => sender.cast(ServerOp::Ping)?,

                // route rest to relay
                ClientOp::Publish { .. } => {
                    //
                    // sender.cast(Message::Outgoing(ServerOp::Ok))?;
                }
                ClientOp::Subscribe { .. } => {
                    //
                    // sender.cast(Message::Outgoing(ServerOp::Ok))?;
                }
                ClientOp::Unsubscribe { .. } => {
                    //
                    // sender.cast(Message::Outgoing(ServerOp::Ok))?;
                }
            };

            Ok(())
        }
    }
}
