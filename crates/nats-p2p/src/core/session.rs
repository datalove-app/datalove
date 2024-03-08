use super::{codec::Codec, ClientOp, ConnectInfo, Protocol, Relay, ServerInfo, ServerOp};
use crate::Error;
use async_nats::{HeaderMap, Message, StatusCode};
use futures::{SinkExt, StreamExt, TryStreamExt};
use ractor::{port::RpcReplyPort, Actor, ActorProcessingErr, ActorRef, SupervisionEvent};
use std::{fmt, marker::PhantomData, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    task::JoinHandle,
};
use tokio_util::codec::{FramedRead, FramedWrite};
use tracing::Instrument;

/// Core API Message.
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

pub type SessionArgs<T> = (T, Relay, Arc<ContextData>);

/// Runtime state of the session.
#[derive(Debug)]
pub struct Context {
    session: ActorRef<CoreMessage>,
    requester: ActorRef<ClientOp>,
    responder: ActorRef<ServerOp>,
    server_info_timer: Option<JoinHandle<()>>,
    // shutdown: Option<JoinHandle<()>>,
    data: Arc<ContextData>,
    // peer_addr: SocketAddr,
    // host_addr: SocketAddr,
}

impl Context {
    fn update_connect_info(&mut self, connect_info: ConnectInfo) {
        Arc::get_mut(&mut self.data).map(|d| d.connect_info = connect_info);
    }

    fn update_server_info(&mut self, server_info: ServerInfo) {
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

/// Mutable configuration state for the session.
#[derive(Debug)]
pub struct ContextData {
    request_timeout: Option<Duration>,
    connect_info: ConnectInfo,
    server_info: ServerInfo,
}

impl ContextData {
    fn client_id(&self) -> u64 {
        self.server_info.client_id
    }
    fn verbose(&self) -> bool {
        self.connect_info.verbose
    }

    pub fn session(&self) -> ActorRef<CoreMessage> {
        ActorRef::where_is(self.session_name()).expect("session actor should be running")
    }
    pub fn session_name(&self) -> String {
        format!("client-session-{}", self.client_id())
    }
    pub fn recv_name(&self) -> String {
        format!("client-session-{}-recv", self.client_id())
    }
    pub fn send_name(&self) -> String {
        format!("client-session-{}-send", self.client_id())
    }

    const BOLD: anstyle::Style = anstyle::Style::new().bold();
    const IP: anstyle::Style =
        anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan)));
    const ID: anstyle::Style =
        anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan)));

    fn trace_header(&self) -> String {
        format_args!(
            "{ip_style}{}{ip_style:#} - {id_style}cid:{}{id_style:#}",
            self.server_info.client_ip,
            self.server_info.client_id,
            ip_style = Self::IP,
            id_style = Self::ID
        )
        .to_string()
    }

    fn trace_incoming(&self, op: &ClientOp) {
        let header = self.trace_header();
        tracing::trace!("{header} - <<- [{op}]",);

        match &op {
            ClientOp::Publish { payload, .. } => {
                tracing::trace!(
                    "{header} - <<- MSG_PAYLOAD: [...]",
                    // payload
                )
            }
            _ => {}
        }
    }

    fn trace_outgoing(&self, op: &ServerOp) {
        let header = self.trace_header();
        tracing::trace!("{header} - ->> [{op}]",);
    }
}

impl<T: util::Split> Session<T> {
    pub fn new() -> Self {
        Self { _io: PhantomData }
    }

    pub async fn run(io: T, relay: Relay, server_info: ServerInfo) -> Result<(), Error> {
        let this = Self::new();
        let data = Arc::new(ContextData {
            request_timeout: None,
            connect_info: CoreMessage::default_connect_info(),
            server_info,
        });

        let (_, handle) = Actor::spawn(Some(data.session_name()), this, (io, relay, data))
            .await
            .map_err(|e| Error::server("Session spawn error", e))?;
        handle
            .await
            .map_err(|e| Error::server("Session run error", e))?;

        Ok(())
    }

    async fn spawn_children(
        &self,
        myself: ActorRef<CoreMessage>,
        io: T,
        relay: Relay,
        data: Arc<ContextData>,
    ) -> Result<(ActorRef<ClientOp>, ActorRef<ServerOp>), Error> {
        let (reader, writer) = io.split();
        let stream = FramedRead::new(reader, Codec::<ClientOp>::default());
        let sink = FramedWrite::new(writer, Codec::<ServerOp>::default());

        let (responder, _) = Actor::spawn_linked(
            Some(data.send_name()),
            responder::Responder::new(),
            (data.clone(), relay.clone(), sink),
            myself.get_cell(),
        )
        .await?;

        let (requester, _) = Actor::spawn_linked(
            Some(data.recv_name()),
            requester::Requester::new(),
            (data.clone(), relay.clone(), stream, responder.clone()),
            myself.get_cell(),
        )
        .await?;

        Ok((requester, responder))
    }
}

impl<T> Actor for Session<T>
where
    T: util::Split,
{
    type Msg = CoreMessage;
    type State = Context;
    type Arguments = SessionArgs<T>;

    /// Spawns sender and receiver actors from [`Framed`] IO parts.
    ///
    /// [`Framed`]: tokio_util::codec::Framed
    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        (io, relay, data): Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let (requester, responder) = self
            .spawn_children(myself.clone(), io, relay, data.clone())
            .await?;

        let server_info = data.server_info.clone();
        let mut state = Context {
            session: myself,
            requester,
            responder,
            server_info_timer: None,
            data,
        };
        state.update_server_info(server_info);
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
        tracing::warn!(
            "Client session closed for {}",
            state.data.server_info.client_ip
        );
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

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            // todo: reset shutdown timer
            CoreMessage::Incoming(op, reply) => match &op {
                // update client connect info, if provided
                ClientOp::Connect(info) => {
                    if let Some(connect_info) = info {
                        state.update_connect_info(connect_info.clone());
                    }
                    reply.send(())?;
                }
                _ => state.requester.cast(op)?,
            },
            CoreMessage::Outgoing(op) => match &op {
                // update server info, (re)start timer
                ServerOp::Info(server_info) => {
                    state.update_server_info(server_info.clone());
                }
                _ => state.responder.cast(op)?,
            },
        }

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
        Arc<ContextData>,
        Relay,
        FramedRead<S, Codec<ClientOp>>,
        ActorRef<ServerOp>,
    );

    pub struct RequesterState {
        responder: ActorRef<ServerOp>,
        framed_read: Option<JoinHandle<Result<(), Error>>>,
        data: Arc<ContextData>,
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
            (data, relay, stream, responder): Self::Arguments,
        ) -> Result<Self::State, ActorProcessingErr> {
            // map read stream to messages
            // send connect msgs to session to be cached
            let framed_read = tokio::task::spawn(async move {
                stream
                    .try_for_each_concurrent(None, |op| async {
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
                    let _ = ractor::call!(state.data.session(), CoreMessage::Incoming, msg)?;

                    if state.data.verbose() {
                        let _ = state.responder.cast(ServerOp::Ok)?;
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
                        let _ = state.responder.cast(ServerOp::Ok)?;
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
                        let _ = state.responder.cast(ServerOp::Ok)?;
                    }
                }
                ClientOp::Unsubscribe { sid, max_msgs } => {
                    //
                    state
                        .relay
                        .unsubscribe((state.data.client_id(), sid), max_msgs, None)?;

                    if state.data.verbose() {
                        let _ = state.responder.cast(ServerOp::Ok)?;
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

    type ResponderArgs<S> = (Arc<ContextData>, Relay, FramedWrite<S, Codec<ServerOp>>);

    pub struct ResponderState<S> {
        framed_write: Option<FramedWrite<S, Codec<ServerOp>>>,
        data: Arc<ContextData>,
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

            let res = state
                .framed_write
                .as_mut()
                .expect("should not handle messages after dropping framed write")
                .send(msg)
                .await?;

            Ok(res)
        }
    }
}

mod util {
    use tokio::{
        io::{AsyncRead, AsyncWrite},
        net::{tcp, TcpStream},
    };

    /// Defines a specific [`split`] for a [`AsyncRead`]+[`AsyncWrite`] stream.
    ///
    /// [`split`]:
    pub trait Split: Send + Sync + 'static {
        type Read: AsyncRead + Send + Sync + Unpin + 'static;
        type Write: AsyncWrite + Send + Sync + Unpin + 'static;

        fn split(self) -> (Self::Read, Self::Write);
    }

    impl Split for TcpStream {
        type Read = tcp::OwnedReadHalf;
        type Write = tcp::OwnedWriteHalf;

        fn split(self) -> (Self::Read, Self::Write) {
            self.into_split()
        }
    }
}
