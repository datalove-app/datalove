use super::{codec::Codec, ConnectInfo, Protocol, Relay, ServerInfo, SubscriberId};
use crate::{Error, Subject};
use async_nats::{HeaderMap, Message, StatusCode};
use bytes::Bytes;
use futures::{future, SinkExt, StreamExt, TryStreamExt};
use ractor::{port::RpcReplyPort, Actor, ActorProcessingErr, ActorRef, SupervisionEvent};
use std::{fmt, marker::PhantomData, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    task::JoinHandle,
};
use tokio_util::codec::{FramedRead, FramedWrite};

/// `ClientOp` represents all actions of `Client`.
///
/// [Original documentation](https://docs.nats.io/reference/reference-protocols/nats-protocol)
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClientOp {
    /// `CONNECT {["option_name":option_value],...}`
    Connect(Option<ConnectInfo>),

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
        let ctrl = self.control();
        match self {
            ClientOp::Ping | ClientOp::Pong => f.write_str(ctrl),
            ClientOp::Connect(info) => write!(
                f,
                "{ctrl} {}",
                serde_json::to_string(info).map_err(|_| fmt::Error)?
            ),
            ClientOp::Info(info) => write!(
                f,
                "{ctrl} {}",
                serde_json::to_string(info).map_err(|_| fmt::Error)?
            ),
            ClientOp::Publish {
                subject,
                reply_to,
                payload,
                ..
            } => {
                write!(
                    f,
                    "{ctrl} {}{} {}",
                    subject,
                    reply_to.as_ref().map_or("".into(), |r| format!(" {}", r)),
                    payload.len(),
                )
            }
            ClientOp::Subscribe {
                sid,
                subject,
                queue_group,
            } => {
                write!(
                    f,
                    "{ctrl} {}{} {}",
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
                    "{ctrl} {}{}",
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
            ServerOp::Ok | ServerOp::Ping | ServerOp::Pong => f.write_str(ctrl),
            ServerOp::Connect(info) => write!(
                f,
                "{ctrl} {}",
                serde_json::to_string(info).map_err(|_| fmt::Error)?
            ),
            ServerOp::Info(info) => write!(
                f,
                "{ctrl} {}",
                serde_json::to_string(info).map_err(|_| fmt::Error)?
            ),
            ServerOp::Err(err) => write!(f, "{ctrl} {err}"),
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
                payload,
                ..
            } => {
                write!(
                    f,
                    "{ctrl} {subject}{}{}{}{} {}",
                    account
                        .as_ref()
                        .map_or(format!(" {sid}"), |a| format!(" {a}")),
                    reply_to.as_ref().map_or("".into(), |r| format!(" {r}")),
                    status.as_ref().map_or("".into(), |s| format!(" {s}")),
                    description.as_ref().map_or("".into(), |d| format!(" {d}")),
                    // headers,
                    payload.len(),
                )
            }
        }
    }
}

impl From<(SubscriberId, Message)> for ServerOp {
    fn from(((_, sid), msg): (SubscriberId, Message)) -> Self {
        Self::from((sid, msg))
    }
}

impl From<(u64, Message)> for ServerOp {
    fn from((sid, msg): (u64, Message)) -> Self {
        Self::Message {
            sid,
            account: None,
            subject: msg.subject,
            reply_to: msg.reply,
            headers: msg.headers,
            status: msg.status,
            description: msg.description,
            payload: msg.payload,
        }
    }
}

impl From<(String, Message)> for ServerOp {
    fn from((account, msg): (String, Message)) -> Self {
        Self::Message {
            sid: 0,
            account: Some(account),
            subject: msg.subject,
            reply_to: msg.reply,
            headers: msg.headers,
            status: msg.status,
            description: msg.description,
            payload: msg.payload,
        }
    }
}

#[derive(Debug)]
pub enum CoreMessage {
    Incoming(ClientOp, RpcReplyPort<()>),
    Outgoing(ServerOp),
}

impl CoreMessage {
    pub fn default_connect_info() -> ConnectInfo {
        ConnectInfo {
            verbose: false,
            pedantic: true,
            user_jwt: None,
            nkey: None,
            signature: None,
            name: None,
            echo: true,
            lang: "".to_string(),
            version: "".to_string(),
            protocol: Protocol::Dynamic,
            tls_required: false,
            user: None,
            pass: None,
            auth_token: None,
            headers: true,
            no_responders: false,
        }
    }
}

/*
 * Actors
 */

#[derive(Debug)]
pub struct Session<T> {
    _io: PhantomData<T>,
}

impl<T> Session<T> {
    pub fn new() -> Self {
        Self { _io: PhantomData }
    }
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

#[derive(Debug)]
pub struct SessionArgs<T> {
    pub io: T,
    pub inbox_prefix: Option<String>,
    pub server_info: ServerInfo,
    pub relay: Relay,
}

#[derive(Debug)]
pub struct SessionState {
    requester: ActorRef<ClientOp>,
    responder: ActorRef<ServerOp>,
    server_info_timer: Option<JoinHandle<()>>,
    // shutdown: Option<JoinHandle<()>>,
    data: Arc<SessionData>,
    // peer_addr: SocketAddr,
    // host_addr: SocketAddr,
}

impl SessionState {
    fn set_connect_info(&mut self, connect_info: ConnectInfo) {
        Arc::get_mut(&mut self.data).map(|d| d.connect_info = connect_info);
    }

    fn set_server_info(&mut self, server_info: ServerInfo) {
        if let Some(ref mut timer) = self.server_info_timer {
            timer.abort();
        }
        Arc::get_mut(&mut self.data).map(|d| d.server_info = server_info.clone());
        self.server_info_timer = Some(
            self.responder
                .send_interval(Duration::from_secs(60), move || {
                    ServerOp::Info(server_info.clone())
                }),
        );
    }
}

#[derive(Debug)]
pub struct SessionData {
    session: ActorRef<CoreMessage>,
    inbox_prefix: String,
    request_timeout: Option<Duration>,
    connect_info: ConnectInfo,
    server_info: ServerInfo,
}

impl SessionData {
    fn client_id(&self) -> u64 {
        self.server_info.client_id
    }
    fn verbose(&self) -> bool {
        self.connect_info.verbose
    }

    fn trace_incoming(&self, op: &ClientOp) {
        tracing::trace!(
            "{} - cid:{} - <<- [{}]",
            self.server_info.client_ip,
            self.server_info.client_id,
            op
        );

        match &op {
            ClientOp::Publish { payload, .. } => {
                tracing::trace!(
                    "{} - cid:{} - <<- MSG_PAYLOAD: [...]",
                    self.server_info.client_ip,
                    self.server_info.client_id,
                    // payload
                )
            }
            _ => {}
        }
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

impl<T> Actor for Session<T>
where
    T: AsyncRead + AsyncWrite + Send + Sync + Unpin + 'static,
{
    type Msg = CoreMessage;
    type State = SessionState;
    type Arguments = SessionArgs<T>;

    /// Spawns sender and receiver actors from [`Framed`] IO parts.
    ///
    /// [`Framed`]: tokio_util::codec::Framed
    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        SessionArgs {
            io,
            inbox_prefix,
            server_info,
            relay,
        }: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let data = Arc::new(SessionData {
            session: myself.clone(),
            inbox_prefix: inbox_prefix.unwrap_or_else(|| "_INBOX".into()),
            request_timeout: None,
            connect_info: CoreMessage::default_connect_info(),
            server_info: server_info.clone(),
        });

        let (requester, responder) = {
            let (reader, writer) = tokio::io::split(io);
            let stream = FramedRead::new(reader, Codec::<ClientOp>::default());
            let sink = FramedWrite::new(writer, Codec::<ServerOp>::default());

            let (responder, _) = Actor::spawn_linked(
                Some(Self::send_name(data.client_id())),
                responder::Responder::new(),
                (data.clone(), relay.clone(), sink),
                myself.get_cell(),
            )
            .await?;

            let (requester, _) = Actor::spawn_linked(
                Some(Self::recv_name(data.client_id())),
                requester::Requester::new(),
                (data.clone(), relay.clone(), stream, responder.clone()),
                myself.get_cell(),
            )
            .await?;

            (requester, responder)
        };

        let mut state = SessionState {
            requester,
            responder,
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
            .responder
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
            CoreMessage::Incoming(op, reply) => match &op {
                // update client connect info
                ClientOp::Connect(Some(connect_info)) => {
                    state.set_connect_info(connect_info.clone());
                    tracing::info!("updated client info");
                    reply.send(())?;
                }
                _ => state.requester.cast(op)?,
            },
            CoreMessage::Outgoing(op) => match &op {
                // update server info, (re)start timer
                ServerOp::Info(server_info) => {
                    state.set_server_info(server_info.clone());
                }
                _ => state.responder.cast(op)?,
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
        let requester_id = state.requester.get_id();
        let responder_id = state.responder.get_id();
        match msg {
            SupervisionEvent::ActorPanicked(actor, msg) if actor.get_id() == requester_id => {
                tracing::error!("Requester panicked: {msg}");
                myself.stop(Some("child panic".to_string()))
            }
            SupervisionEvent::ActorPanicked(actor, msg) if actor.get_id() == responder_id => {
                tracing::error!("Responder panicked: {msg}");
                myself.stop(Some("child panic".to_string()))
            }
            SupervisionEvent::ActorPanicked(_, msg) => {
                tracing::error!("Unknown actor panicked: {msg}");
            }
            SupervisionEvent::ActorTerminated(actor, _, reason)
                if actor.get_id() == requester_id =>
            {
                tracing::error!("Requester terminated: {}", reason.unwrap_or_default());
                myself.stop(Some("child terminated".to_string()))
            }
            SupervisionEvent::ActorTerminated(actor, _, reason)
                if actor.get_id() == responder_id =>
            {
                tracing::error!("Responder terminated: {}", reason.unwrap_or_default());
                myself.stop(Some("child terminated".to_string()))
            }
            SupervisionEvent::ActorTerminated(_, _, reason) => {
                tracing::error!("Unknown actor terminated: {}", reason.unwrap_or_default());
            }
            _ => {
                // tracing::warn!("Unhandled supervisor event: {:?}", msg);
            }
        };

        Ok(())
    }
}

mod requester {
    use super::*;

    #[derive(Debug)]
    pub struct Requester<S>(PhantomData<S>);

    impl<S> Requester<S> {
        pub fn new() -> Self {
            Self(PhantomData)
        }
    }

    type RequesterArgs<S> = (
        Arc<SessionData>,
        Relay,
        FramedRead<S, Codec<ClientOp>>,
        ActorRef<ServerOp>,
    );

    pub struct RequesterState {
        responder: ActorRef<ServerOp>,
        framed_read: Option<JoinHandle<Result<(), Error>>>,
        data: Arc<SessionData>,
        relay: Relay,
    }

    impl<S> Actor for Requester<S>
    where
        S: AsyncRead + Send + Sync + Unpin + 'static,
    {
        type Msg = ClientOp;
        type State = RequesterState;
        type Arguments = RequesterArgs<S>;

        async fn pre_start(
            &self,
            myself: ActorRef<Self::Msg>,
            (data, relay, mut stream, responder): Self::Arguments,
        ) -> Result<Self::State, ActorProcessingErr> {
            // map read stream to messages
            // send connect msgs to session to be cached
            let framed_read = tokio::spawn(async move {
                stream
                    .try_for_each(|op| async {
                        myself.cast(op)?;
                        Ok(())
                    })
                    .await?;
                Ok(())
            });

            Ok(RequesterState {
                data,
                relay,
                framed_read: Some(framed_read),
                responder,
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

            match msg {
                // route quick messages directly to sender
                ClientOp::Connect(_) => {
                    // send connect to session to cache connect info
                    let _ = ractor::call!(state.data.session, CoreMessage::Incoming, msg)?;

                    if state.data.verbose() {
                        state.responder.cast(ServerOp::Ok)?;
                    }
                }
                ClientOp::Info(_) => state
                    .responder
                    .cast(ServerOp::Info(state.data.server_info.clone()))?,
                ClientOp::Ping => state.responder.cast(ServerOp::Pong)?,
                ClientOp::Pong => state.responder.cast(ServerOp::Ping)?,

                // route rest to relay
                ClientOp::Publish {
                    subject,
                    reply_to,
                    headers,
                    payload,
                } => {
                    // publish message to relay
                    let _ = state.relay.publish(
                        state.data.client_id(),
                        Message {
                            subject,
                            reply: reply_to,
                            headers,
                            status: None,
                            description: None,
                            length: payload.len(),
                            payload,
                        },
                    )?;

                    if state.data.verbose() {
                        state.responder.cast(ServerOp::Ok)?;
                    }
                }
                ClientOp::Subscribe {
                    subject,
                    queue_group,
                    sid,
                } => {
                    //
                    let _ = state
                        .relay
                        .subscribe(
                            (state.data.client_id(), sid),
                            subject,
                            queue_group,
                            state.responder.clone(),
                        )
                        .await?;

                    if state.data.verbose() {
                        state.responder.cast(ServerOp::Ok)?;
                    }
                }
                ClientOp::Unsubscribe { sid, max_msgs } => {
                    //
                    state
                        .relay
                        .unsubscribe((state.data.client_id(), sid), max_msgs, None)?;

                    if state.data.verbose() {
                        state.responder.cast(ServerOp::Ok)?;
                    }
                }
            };

            Ok(())
        }
    }
}

mod responder {
    use super::*;

    #[derive(Debug)]
    pub struct Responder<S> {
        _sink: PhantomData<S>,
    }

    impl<S> Responder<S> {
        pub fn new() -> Self {
            Self { _sink: PhantomData }
        }
    }

    type ResponderArgs<S> = (Arc<SessionData>, Relay, FramedWrite<S, Codec<ServerOp>>);

    pub struct ResponderState<S> {
        framed_write: Option<FramedWrite<S, Codec<ServerOp>>>,
        data: Arc<SessionData>,
        relay: Relay,
    }

    impl<S> Actor for Responder<S>
    where
        S: AsyncWrite + Send + Sync + Unpin + 'static,
    {
        type Msg = ServerOp;
        type State = ResponderState<S>;
        type Arguments = ResponderArgs<S>;

        async fn pre_start(
            &self,
            _myself: ActorRef<Self::Msg>,
            (data, relay, framed_write): Self::Arguments,
        ) -> Result<Self::State, ActorProcessingErr> {
            Ok(ResponderState {
                data,
                relay,
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
