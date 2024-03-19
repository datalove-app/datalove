pub mod bind;
pub mod kvs;
pub mod pubsub;

use async_nats::{ConnectInfo, Protocol};

use self::bind::bind_tcp;
use crate::{
    core::{message::Codec, ClientOp, CoreMessage, ServerOp},
    server::*,
    Config, Error,
};
use hydroflow::{hydroflow_syntax, scheduled::graph::Hydroflow, DemuxEnum};
use iroh_net::NodeId;
use std::{
    cell::RefCell,
    collections::HashMap,
    net::SocketAddr,
    rc::Rc,
    time::{Duration, Instant},
};

type TcpMsg = bind::Msg<CoreMessage>;
type Msg<T> = (ClientId, T);
type Sessions = HashMap<ClientId, ConnectInfo>;

#[derive(Copy, Clone, Debug, DemuxEnum, PartialEq, Eq, Hash)]
pub enum ClientId {
    Local(u64, SocketAddr),
    Remote(u64, SocketAddr, NodeId),
}
impl ClientId {
    pub fn cid(&self) -> u64 {
        match self {
            Self::Local(cid, _) => *cid,
            Self::Remote(cid, _, _) => *cid,
        }
    }
    pub fn addr(&self) -> SocketAddr {
        match self {
            Self::Local(_, addr) => *addr,
            Self::Remote(_, addr, _) => *addr,
        }
    }
    pub(crate) fn trace(&self, msg: &CoreMessage) {
        let (cid, addr) = (self.cid(), self.addr());
        msg.trace(addr, cid)
    }
}
impl From<(u64, SocketAddr)> for ClientId {
    fn from((cid, addr): (u64, SocketAddr)) -> Self {
        Self::Local(cid, addr)
    }
}

fn default_connect_info() -> ConnectInfo {
    ConnectInfo {
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
    }
}

pub async fn server(mut config: Config) -> Result<Hydroflow<'static>, Error> {
    let sk = config.read_ssh_key().await?;

    // start pubsub service
    let (pubsub_send, pubsub_recv) = pubsub::run(None).await?;
    // start kvs service
    // let (kvs_send, kvs_recv) = kvs::run(None).await?;
    // start object store service

    // start client connection listener
    let (egress, ingress, addr) =
        bind_tcp::<Codec>(config.listen_addr(), Default::default()).await?;

    // core server configuration and runtime data
    // must be cloned for each instance its used w/in the flow
    let server_data = Rc::new(RefCell::new(
        ServerData::new(sk.public(), config).with_host_addr(addr),
    ));
    let server_data1 = server_data.clone();
    // let server_data2 = server_data.clone();

    // session map
    let sessions = Rc::new(RefCell::new(Sessions::new()));
    let sessions1 = sessions.clone();
    let sessions2 = sessions.clone();

    let hf = hydroflow_syntax! {
        // initialize server data
        // log start message
        initialize() -> for_each(|_| server_data1.borrow().log_start_message());

        /******************** deps ********************/
        pubsub_reqs = union() -> dest_sink(pubsub_send.into_sink());
        pubsub_resp = source_stream(pubsub_recv.into_stream()) -> tee();

        /******************** timers ********************/
        // send ServerInfo to clients every 60 seconds
        source_interval(Duration::from_secs(60))
            -> flat_map(|_| sessions1.borrow().keys().copied().collect::<Vec<ClientId>>())
            -> outbound_server_info[interval];

        // TODO: shutdown client after ... minutes of inactivity

        /******************** network inbound ********************/
        inbound = source_stream(ingress.into_stream()) -> demux_enum::<TcpMsg>();

        /*
         * connect/disconnect
         */
        conns = inbound[Connect]
            -> map(ClientId::from)
            -> tee();
        conns[responses] -> outbound_server_info[on_connect];
        conns[store] -> for_each(|id| {
            sessions2.borrow_mut().insert(id, default_connect_info());
        });
        inbound[Disconnect]
            -> map(|(id, err)| (ClientId::from(id), err))
            -> inspect(|(id, error)| tracing::info!("client {} disconnected with error: {error}", id.cid()))
            -> for_each(|(id, _)| { sessions2.borrow_mut().remove(&id); });

        inbound[Frame]
            -> map(|(id, frame)| (ClientId::from(id), frame))
            -> inspect(|(id, msg)| id.trace(msg))
            // -> inspect(|((cid, addr), msg)| msg.trace(*addr, *cid))
            -> cmds;

        /********** commands **********/
        cmds = map(|(id, msg): (ClientId, CoreMessage)| (id, msg.unwrap_inbound(), Instant::now()))
            -> demux(|(id, msg, _ts), var_args!(Conn, Info, Ping, Pong, Pub, Sub, Unsub)| match msg {
                ClientOp::Connect(info) => Conn.give((id, info)),
                ClientOp::Info(info) => Info.give((id, info)),
                ClientOp::Ping => Ping.give(id),
                ClientOp::Pong => Pong.give(id),
                ClientOp::Publish { .. } => Pub.give((id, msg.unwrap_into_message())),
                ClientOp::Subscribe { subject, queue_group, sid } => Sub.give((id, subject, queue_group, sid)),
                ClientOp::Unsubscribe { sid, max_msgs } => Unsub.give((id, sid, max_msgs))
            });

        /*
         * connect
         */
        connect_info = cmds[Conn]
            -> filter_map(|(id, info): Msg<Option<ConnectInfo>>| info.map(|info| (id, info)))
            -> tee();
        connect_info[store] -> for_each(|(id, info): Msg<ConnectInfo>| {
            sessions2.borrow_mut().insert(id, info);
        });
        connect_info[responses]  // only send response if client is verbose
            -> filter_map(|(id, info)| info.verbose.then_some(id))
            -> outbound_server_info;

        /*
         * info, ping/pong
         */
        cmds[Info] -> map(|(id, _)| id) -> outbound_server_info;
        cmds[Ping] -> map(|id| (id, ServerOp::Pong)) -> outbound;
        cmds[Pong] -> map(|id| (id, ServerOp::Ping)) -> outbound;

        /*
         * publish
         */
        cmds[Pub]
            -> map(|(id, msg)| (id, pubsub::Command::Pub(msg)))
            -> pubsub_reqs;

        /*
         * subscribe
         */
        cmds[Sub]
            -> map(|(id, subject, queue_group, sid)| (id, pubsub::Command::Sub { subject, queue_group, sid }))
            -> pubsub_reqs;

        /*
         * unsubscribe
         */
        cmds[Unsub]
            -> map(|(id, sid, max_msgs)| (id, pubsub::Command::Unsub { sid, max_msgs }))
            -> pubsub_reqs;

        /******************** network outbound ********************/
        outbound_server_info = union()
            -> identity::<ClientId>()
            -> map(|id| (id, server_data.borrow().server_info_for_client(id.cid(), id.addr())))
            -> map(|(id, info)| (id, ServerOp::Info(info)))
            -> outbound[server_info];

        // outbound_pubsub = pubsub_resp
        //     -> map(|(cid, msg)| {
        //         sessions2.borrow().get(&cid).map(|info| info.client_id);
        //     })

        outbound = union()
            -> identity::<Msg<ServerOp>>()
            -> map(|(id, msg)| (id, CoreMessage::Outbound(msg)))
            -> inspect(|(id, msg)| id.trace(msg))
            -> map(|(id, msg)| (id.cid(), msg))
            -> dest_sink(egress.into_sink());
    };

    {
        // let mut f = fs::File::options()
        //     .create(true)
        //     .write(true)
        //     .open("hydroflow.mmd")
        //     .await?;
        // let mut fmt = String::new();
        // hf.meta_graph()
        //     .expect("failed to parse hydroflow graph")
        //     .write_mermaid(
        //         &mut fmt,
        //         &WriteConfig {
        //             ..Default::default()
        //         },
        //     )
        //     .expect("failed to write hydroflow mermaid");
        // f.write_all(fmt.as_bytes()).await?;
    }

    Ok(hf)
}
