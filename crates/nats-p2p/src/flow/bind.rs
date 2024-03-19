use flume::{unbounded, Receiver, Sender};
use futures::{SinkExt, StreamExt, TryStreamExt};
use hydroflow::{util::tcp_framed, DemuxEnum};
use std::{cell::RefCell, collections::HashMap, error, fmt, net::SocketAddr, pin::pin, rc::Rc};
use tokio::{io, net::TcpListener, task::spawn_local};
use tokio_stream::wrappers::TcpListenerStream;
use tokio_util::codec::{Decoder, Encoder, FramedWrite};

pub type ClientId = (u64, SocketAddr);
// pub type Frame<T> = (ClientId, T);
pub type FramedSink<T> = Sender<(u64, T)>;
pub type FramedStream<T> = Receiver<Msg<T>>;

#[derive(Debug, Clone, DemuxEnum)]
pub enum Msg<T> {
    Connect(ClientId),
    Frame(ClientId, T),
    Disconnect(ClientId, String), // + error msg?
}

/// Create a listening tcp socket, and then as new connections come in, receive their data and forward it to a queue.
pub async fn bind_tcp<C>(
    // handle: LocalPoolHandle,
    endpoint: SocketAddr,
    codec: C,
) -> Result<(FramedSink<C::Item>, FramedStream<C::Item>, SocketAddr), io::Error>
where
    C: Clone + Decoder + Encoder<C::Item> + 'static,
    C::Item: 'static,
    <C as Decoder>::Error: error::Error + fmt::Debug,
{
    let listener = TcpListener::bind(endpoint).await?;
    let bound_endpoint = listener.local_addr()?;
    let mut listener = TcpListenerStream::new(listener);

    let (egress_send, egress_recv) = unbounded();
    let (ingress_send, ingress_recv) = unbounded();

    let clients = Rc::new(RefCell::new(
        HashMap::<u64, (SocketAddr, FramedWrite<_, C>)>::new(),
    ));
    // let clients = Arc::new(DashMap::<TcpClientId, FramedWrite<_, C>>::new());

    spawn_local({
        let clients = clients.clone();

        async move {
            while let Some((id, payload)) = egress_recv.stream().next().await {
                let entry = clients.borrow_mut().remove(&id);
                if let Some((addr, mut sender)) = entry {
                    let _ = SinkExt::send(&mut sender, payload).await;
                    clients.borrow_mut().insert(id, (addr, sender));
                }
            }
        }
    });

    spawn_local(async move {
        let mut client_id = 0;
        loop {
            let (stream, peer_addr) = if let Ok(Some(stream)) = listener.try_next().await {
                if let Ok(peer_addr) = stream.peer_addr() {
                    (stream, peer_addr)
                } else {
                    continue;
                }
            } else {
                continue;
            };

            let ingress_send = ingress_send.clone();
            let (send, recv) = tcp_framed(stream, codec.clone());

            client_id += 1;
            clients.borrow_mut().insert(client_id, (peer_addr, send));

            spawn_local({
                let clients = clients.clone();
                async move {
                    let mapped = recv.filter_map(|res| async {
                        // Ok(x.map(|x| Msg::Frame((client_id, peer_addr), x)))
                        match res {
                            Ok(x) => Some(Ok(Msg::Frame((client_id, peer_addr), x))),
                            Err(e) => {
                                tracing::error!("error reading from client: {:?}", e);
                                clients.borrow_mut().remove(&client_id);
                                Some(Ok(Msg::Disconnect((client_id, peer_addr), e.to_string())))
                            }
                        }
                    });

                    let _ = ingress_send
                        .sink()
                        .send(Msg::Connect((client_id, peer_addr)))
                        .await;
                    let _ = ingress_send.sink().send_all(&mut pin!(mapped)).await;

                    clients.borrow_mut().remove(&client_id);
                }
            });
        }
    });

    Ok((egress_send, ingress_recv, bound_endpoint))
}
