pub use self::{
    receiver::{Receiver, ReceiverArgs, ReceiverState},
    sender::Sender,
    util::*,
};

use super::{
    codec::Codec, ClientOp, ConnectInfo, CoreMessage, Message, Protocol, Relay, ServerInfo,
    ServerOp, StatusCode,
};
use crate::Error;
use bytes::Bytes;
use futures::{SinkExt, Stream, StreamExt, TryStream, TryStreamExt};

use ractor::{Actor, ActorProcessingErr, ActorRef, SupervisionEvent};
use std::{
    fmt,
    marker::PhantomData,
    net::SocketAddr,
    sync::{atomic::AtomicU64, Arc},
    time::Duration,
};
use tokio::{
    io::{self, AsyncRead, AsyncWrite},
    sync::RwLock,
    task::JoinHandle,
};
use tokio_util::codec::{FramedRead, FramedWrite};

#[derive(Debug)]
pub struct SessionManager<S> {
    // sessions: Vec<SessionInfo>,
    last_client_id: AtomicU64,
    _session: PhantomData<S>,
}

impl<S> SessionManager<S> {
    pub async fn run<T>(
        relay: Relay,
        server_info: ServerInfo,
        listener: impl Stream<Item = io::Result<T>> + TryStream<Ok = T, Error = io::Error>,
        // shutdown: impl Future,
    ) -> Result<(), Error>
    where
        S: Actor<Arguments = (T, Relay, SessionInfo)> + Default,
        T: util::NetworkSplit,
    {
        let this = Self::default();

        // TODO: shutdown listener
        // TODO: handle errors with sleep and retry
        let info = SessionInfo::new(server_info);
        listener
            .for_each_concurrent(None, |io| async {
                match io {
                    Err(e) => {
                        tracing::error!("Session IO error: {}", e);
                    }
                    Ok(io) => {
                        let client_id = this.next_client_id();
                        let local_addr = io.local_addr().unwrap();
                        let peer_addr = io.peer_addr().unwrap();

                        let info = info
                            .clone()
                            .with_local_addr(local_addr)
                            .with_client(client_id, peer_addr);

                        Self::run_session(io, relay.clone(), info).await.unwrap();
                    }
                }
            })
            .await;
        Ok(())
    }

    async fn run_session<T>(io: T, relay: Relay, session_info: SessionInfo) -> Result<(), Error>
    where
        S: Actor<Arguments = (T, Relay, SessionInfo)> + Default,
        T: util::NetworkSplit,
    {
        let (_, handle) = Actor::spawn(
            Some(session_info.session_name()),
            S::default(),
            (io, relay, session_info),
        )
        .await
        .map_err(|e| Error::server("Session spawn error", e))?;
        handle
            .await
            .map_err(|e| Error::server("Session run error", e))?;

        Ok(())
    }

    fn next_client_id(&self) -> u64 {
        self.last_client_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}

impl<S> Default for SessionManager<S> {
    fn default() -> Self {
        Self {
            last_client_id: AtomicU64::new(1),
            _session: PhantomData,
        }
    }
}

/// Ongoing tasks and actors underpinning a [`Session`].
#[derive(Debug)]
pub struct SessionContext {
    session: ActorRef<CoreMessage>,
    receiver: ActorRef<ClientOp>,
    sender: ActorRef<ServerOp>,
    server_info_sender: Option<JoinHandle<()>>,
    // shutdown: Option<JoinHandle<()>>,
    info: Arc<RwLock<SessionInfo>>,
    // peer_addr: SocketAddr,
    // host_addr: SocketAddr,
}

impl SessionContext {
    async fn update_connect_info(&mut self, connect_info: ConnectInfo) {
        self.info.write().await.connect_info = connect_info;
    }

    async fn update_server_info(&mut self, server_info: Option<ServerInfo>) {
        if let Some(server_info) = server_info {
            self.info.write().await.server_info = server_info.clone();
        }

        // // reset server_info timer
        // if let Some(ref mut timer) = self.server_info_sender {
        //     timer.abort();
        // }
        // self.server_info_sender = Some(
        //     self.responder
        //         .send_interval(Duration::from_secs(60), move || {
        //             ServerOp::Info(server_info.clone())
        //         }),
        // );
    }
}

/// Mutable session information.
#[derive(Debug, Clone)]
pub struct SessionInfo {
    request_timeout: Option<Duration>,
    connect_info: ConnectInfo,
    server_info: ServerInfo,
}

impl SessionInfo {
    pub fn new(server_info: ServerInfo) -> Self {
        Self {
            request_timeout: None,
            server_info,
            connect_info: ConnectInfo {
                verbose: false,
                pedantic: true,
                user_jwt: None,
                nkey: None,
                signature: None,
                name: None,
                echo: false,
                lang: "rust".to_string(),
                version: "1.32.0".to_string(),
                protocol: Protocol::Dynamic,
                tls_required: false,
                user: None,
                pass: None,
                auth_token: None,
                headers: true,
                no_responders: true,
            },
        }
    }
    pub fn with_local_addr(mut self, local_addr: SocketAddr) -> Self {
        self.server_info.host = local_addr.ip().to_string();
        self.server_info.port = local_addr.port();
        self
    }
    pub fn with_client(mut self, id: u64, addr: SocketAddr) -> Self {
        self.server_info.client_id = id;
        self.server_info.client_ip = addr.to_string();
        self
    }

    pub fn session(&self) -> ActorRef<CoreMessage> {
        ActorRef::where_is(self.session_name()).expect("session actor should be running")
    }
    pub fn session_name(&self) -> String {
        format!(
            "{}-client-session-{}",
            self.server_id_prefix(),
            self.client_id()
        )
    }
    pub fn recv_name(&self) -> String {
        format!(
            "{}-client-session-{}-recv",
            self.server_id_prefix(),
            self.client_id()
        )
    }
    pub fn send_name(&self) -> String {
        format!(
            "{}-client-session-{}-send",
            self.server_id_prefix(),
            self.client_id()
        )
    }

    pub fn validate_connection(&self, connect_info: &ConnectInfo) -> bool {
        !connect_info.tls_required
    }
}

impl SessionInfo {
    fn echo(&self) -> bool {
        self.connect_info.echo
    }
    fn no_responders(&self) -> bool {
        self.connect_info.no_responders
    }
    fn verbose(&self) -> bool {
        self.connect_info.verbose
    }
    fn client_id(&self) -> u64 {
        self.server_info.client_id
    }
    fn server_id_prefix(&self) -> &str {
        &self.server_info.server_id[..6]
    }
}

pub struct Session<T: util::Split, R = Receiver<<T as util::Split>::Read>> {
    _io: PhantomData<T>,
    _recv: PhantomData<R>,
}

impl<T: util::Split, R> fmt::Debug for Session<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Session<{}, {}>",
            core::any::type_name::<T>(),
            core::any::type_name::<R>()
        )
    }
}
impl<T: util::Split, R: Default> Default for Session<T, R> {
    fn default() -> Self {
        Self {
            _io: PhantomData,
            _recv: PhantomData,
        }
    }
}

impl<T, R> Actor for Session<T, R>
where
    T: util::Split,
    R: Actor<Msg = ClientOp, Arguments = ReceiverArgs<T::Read>> + Default,
{
    type Msg = CoreMessage;
    type State = SessionContext;
    type Arguments = (T, Relay, SessionInfo);

    /// Spawns sender and receiver actors from [`Framed`] IO parts.
    ///
    /// [`Framed`]: tokio_util::codec::Framed
    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        (io, relay, info): Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let (recv_name, send_name) = (info.recv_name(), info.send_name());
        let info = Arc::new(RwLock::new(info));

        let (receiver, sender) = {
            let (reader, writer) = io.split();
            let stream = FramedRead::new(reader, Codec::<ClientOp>::default());
            let sink = FramedWrite::new(writer, Codec::<ServerOp>::default());

            let (sender, _) = Actor::spawn_linked(
                Some(send_name),
                sender::Sender::new(),
                (info.clone(), relay.clone(), sink),
                myself.get_cell(),
            )
            .await?;

            let (receiver, _) = Actor::spawn_linked(
                Some(recv_name),
                R::default(),
                (
                    stream,
                    info.clone(),
                    relay.clone(),
                    myself.clone(),
                    sender.clone(),
                ),
                myself.get_cell(),
            )
            .await?;
            (receiver, sender)
        };

        let mut state = SessionContext {
            session: myself,
            receiver,
            sender,
            server_info_sender: None,
            info,
        };

        state.update_server_info(None).await;
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
            .cast(ServerOp::Info(state.info.read().await.server_info.clone()))?;

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
            state.info.read().await.server_info.client_ip
        );
        Ok(())
    }

    async fn handle_supervisor_evt(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: SupervisionEvent,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        let requester = state.receiver.get_id();
        let responder = state.sender.get_id();
        match msg {
            SupervisionEvent::ActorPanicked(actor, msg) if actor.get_id() == requester => {
                tracing::error!("Requester panicked: {msg}");
                myself.stop(Some("child panic".to_string()))
            }
            SupervisionEvent::ActorPanicked(actor, msg) if actor.get_id() == responder => {
                tracing::error!("Responder panicked: {msg}");
                myself.stop(Some("child panic".to_string()))
            }
            SupervisionEvent::ActorPanicked(_, msg) => {
                tracing::error!("Unknown actor panicked: {msg}");
            }
            SupervisionEvent::ActorTerminated(actor, _, reason) if actor.get_id() == requester => {
                tracing::error!("Requester terminated: {}", reason.unwrap_or_default());
                myself.stop(Some("child terminated".to_string()))
            }
            SupervisionEvent::ActorTerminated(actor, _, reason) if actor.get_id() == responder => {
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
            CoreMessage::Inbound(op, reply) => match &op {
                // update client connect info, if provided
                // TODO: validate connect info
                ClientOp::Connect(info) => {
                    if let Some((connect_info, reply)) = info.as_ref().zip(reply) {
                        state.update_connect_info(connect_info.clone()).await;
                        reply.send(())?;
                    }
                }
                _ => state.receiver.cast(op)?,
            },
            CoreMessage::Outbound(op) => match &op {
                // update server info, (re)start timer
                ServerOp::Info(server_info) => {
                    state.update_server_info(Some(server_info.clone())).await;
                }
                _ => state.sender.cast(op)?,
            },
        }

        Ok(())
    }
}

mod receiver {
    use super::*;

    #[derive(Debug)]
    pub struct Receiver<R>(PhantomData<R>);

    impl<R> Default for Receiver<R> {
        fn default() -> Self {
            Self(PhantomData)
        }
    }

    pub type ReceiverArgs<R> = (
        FramedRead<R, Codec<ClientOp>>,
        Arc<RwLock<SessionInfo>>,
        Relay,
        ActorRef<CoreMessage>,
        ActorRef<ServerOp>,
    );

    pub struct ReceiverState {
        framed_read: Option<JoinHandle<Result<(), Error>>>,
        data: Arc<RwLock<SessionInfo>>,
        relay: Relay,
        session: ActorRef<CoreMessage>,
        responder: ActorRef<ServerOp>,
    }

    impl<R> Actor for Receiver<R>
    where
        R: AsyncRead + Send + Sync + Unpin + 'static,
    {
        type Msg = ClientOp;
        type State = ReceiverState;
        type Arguments = ReceiverArgs<R>;

        /// Spawn a task to read from the stream and cast messages to the session.
        async fn pre_start(
            &self,
            myself: ActorRef<Self::Msg>,
            (stream, data, relay, session, responder): Self::Arguments,
        ) -> Result<Self::State, ActorProcessingErr> {
            let framed_read = tokio::task::spawn(async move {
                stream
                    .try_for_each_concurrent(None, |op| async {
                        myself.cast(op)?;
                        Ok(())
                    })
                    .await?;
                Ok(())
            });

            Ok(Self::State {
                data,
                relay,
                framed_read: Some(framed_read),
                session,
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

        /// Trace each incoming message, and route it to the relay or responder.
        async fn handle(
            &self,
            _myself: ActorRef<Self::Msg>,
            msg: Self::Msg,
            state: &mut Self::State,
        ) -> Result<(), ActorProcessingErr> {
            let (client_id, verbose, no_responders, echo) = {
                let data = &state.data.read().await;
                // msg.trace(&data.server_info.client_ip, data.server_info.client_id);
                (
                    data.client_id(),
                    data.verbose(),
                    data.no_responders(),
                    data.echo(),
                )
            };

            match msg {
                // route quick messages directly to sender
                ClientOp::Connect(_) => {
                    // send connect to session to cache connect info
                    let _ = state
                        .session
                        .call(|reply| CoreMessage::Inbound(msg, Some(reply)), None)
                        .await?;

                    if state.data.read().await.verbose() {
                        state.responder.cast(ServerOp::Ok)?;
                    }
                }
                ClientOp::Info(_) => {
                    let server_info = &state.data.read().await.server_info;
                    state.responder.cast(ServerOp::Info(server_info.clone()))?
                }
                ClientOp::Ping => state.responder.cast(ServerOp::Pong)?,
                ClientOp::Pong => state.responder.cast(ServerOp::Ping)?,

                // route rest to relay
                ClientOp::Publish {
                    subject,
                    reply: reply_to,
                    headers,
                    payload,
                } => {
                    // publish message to relay
                    let status = state.relay.publish(
                        client_id,
                        Message {
                            subject: subject.clone(),
                            reply: reply_to.clone(),
                            headers,
                            status: None,
                            description: None,
                            length: payload.len(),
                            payload,
                        },
                        echo,
                        Some(state.responder.clone()),
                    )?;

                    match (status, reply_to) {
                        (StatusCode::OK, _) if verbose => {
                            state.responder.cast(ServerOp::Ok)?;
                        }
                        (StatusCode::NO_RESPONDERS, Some(reply)) if no_responders => {
                            state.responder.cast(ServerOp::Message {
                                status: Some(status),
                                subject: reply,
                                reply_to: None,
                                sid: 0,
                                account: None,
                                // headers: None,
                                headers: Some(Default::default()),
                                description: None,
                                payload: Bytes::new(),
                            })?;
                        }
                        _ => {}
                    }
                }
                ClientOp::Subscribe {
                    subject,
                    queue_group,
                    sid,
                } => {
                    //
                    let status = state
                        .relay
                        .subscribe(
                            (client_id, sid),
                            subject,
                            queue_group,
                            state.responder.clone(),
                        )
                        .await?;

                    match status {
                        StatusCode::OK if verbose => {
                            state.responder.cast(ServerOp::Ok)?;
                        }
                        _ => {}
                    }
                }
                ClientOp::Unsubscribe { sid, max_msgs } => {
                    //
                    let status = state.relay.unsubscribe((client_id, sid), max_msgs, None)?;

                    match status {
                        StatusCode::OK if verbose => {
                            state.responder.cast(ServerOp::Ok)?;
                        }
                        _ => {}
                    }
                }
            };

            Ok(())
        }
    }
}

mod sender {
    use super::*;

    #[derive(Debug)]
    pub struct Sender<W> {
        _sink: PhantomData<W>,
    }

    impl<W> Default for Sender<W> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<W> Sender<W> {
        pub fn new() -> Self {
            Self { _sink: PhantomData }
        }
    }

    pub struct ResponderState<W> {
        framed_write: Option<FramedWrite<W, Codec<ServerOp>>>,
        data: Arc<RwLock<SessionInfo>>,
        relay: Relay,
    }

    impl<W> Actor for Sender<W>
    where
        W: AsyncWrite + Send + Sync + Unpin + 'static,
    {
        type Msg = ServerOp;
        type State = ResponderState<W>;
        type Arguments = (
            Arc<RwLock<SessionInfo>>,
            Relay,
            FramedWrite<W, Codec<ServerOp>>,
        );

        async fn pre_start(
            &self,
            _myself: ActorRef<Self::Msg>,
            (data, relay, framed_write): Self::Arguments,
        ) -> Result<Self::State, ActorProcessingErr> {
            Ok(ResponderState {
                framed_write: Some(framed_write),
                data,
                relay,
            })
        }

        /// Drop the framed write on stop.
        async fn post_stop(
            &self,
            _myself: ActorRef<Self::Msg>,
            state: &mut Self::State,
        ) -> Result<(), ActorProcessingErr> {
            drop(state.framed_write.take());
            Ok(())
        }

        /// Trace each outgoing message, and write it to the framed write.
        async fn handle(
            &self,
            _myself: ActorRef<Self::Msg>,
            msg: Self::Msg,
            state: &mut Self::State,
        ) -> Result<(), ActorProcessingErr> {
            {
                let server_info = &state.data.read().await.server_info;
                // msg.trace(&server_info.client_ip, server_info.client_id);
            }

            Ok(state
                .framed_write
                .as_mut()
                .expect("should not handle messages after dropping framed write")
                .send(msg)
                .await?)
        }
    }
}

mod util {
    use std::net::SocketAddr;
    use tokio::{
        io::{self, AsyncRead, AsyncWrite},
        net::{tcp, TcpStream},
    };

    pub trait NetworkSplit: Split {
        fn local_addr(&self) -> io::Result<SocketAddr>;
        fn peer_addr(&self) -> io::Result<SocketAddr>;
    }

    impl NetworkSplit for TcpStream {
        fn local_addr(&self) -> io::Result<SocketAddr> {
            TcpStream::local_addr(self)
        }
        fn peer_addr(&self) -> io::Result<SocketAddr> {
            TcpStream::peer_addr(self)
        }
    }

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
