use hydroflow::scheduled::{graph::Hydroflow, handoff::VecHandoff, port::RecvPort};
use tokio_util::codec::{Decoder, Encoder, FramedRead, FramedWrite};

pub trait NetworkVertex {
    // TODO(justin): this needs to return a result/get rid of all the unwraps, I
    // guess we need a HydroflowError?
    /// Begins listening on some TCP port. Returns an [OutputPort] representing
    /// the stream of messages received. Currently there is no notion of
    /// identity to the connections received, if they are to be attached to some
    /// participant in the system, that needs to be included in the message
    /// directly.
    ///
    /// The messages will be interpreted to be bincode-encoded, length-delimited
    /// messages, as produced by [Self::outbound_tcp_vertex].
    async fn inbound_tcp_vertex<T, C>(
        &mut self,
        port: Option<u16>,
    ) -> (u16, RecvPort<VecHandoff<T>>)
    where
        T: Send + 'static,
        C: Decoder<Item = T> + Default;
}

impl<'a> NetworkVertex for Hydroflow<'a> {
    // TODO(justin): this needs to return a result/get rid of all the unwraps, I
    // guess we need a HydroflowError?
    /// Begins listening on some TCP port. Returns an [OutputPort] representing
    /// the stream of messages received. Currently there is no notion of
    /// identity to the connections received, if they are to be attached to some
    /// participant in the system, that needs to be included in the message
    /// directly.
    ///
    /// The messages will be interpreted to be bincode-encoded, length-delimited
    /// messages, as produced by [Self::outbound_tcp_vertex].
    async fn inbound_tcp_vertex<T, C>(
        &mut self,
        port: Option<u16>,
    ) -> (u16, RecvPort<VecHandoff<T>>)
    where
        T: Send + 'static,
        C: Decoder<Item = T> + Default,
    {
        let listener = TcpListener::bind(format!("localhost:{}", port.unwrap_or(0)))
            .await
            .unwrap();
        let port = listener.local_addr().unwrap().port();

        // TODO(justin): figure out an appropriate buffer here.
        let (incoming_send, incoming_messages) = futures::channel::mpsc::channel(1024);

        // Listen to incoming connections and spawn a tokio task for each one,
        // which feeds into the channel.
        // TODO(justin): give some way to get a handle into this thing.
        tokio::spawn(async move {
            loop {
                let (socket, _) = listener.accept().await.unwrap();
                let (reader, _) = socket.into_split();
                let mut reader = FramedRead::new(reader, C::default());
                let mut incoming_send = incoming_send.clone();
                tokio::spawn(async move {
                    while let Some(msg) = reader.next().await {
                        // TODO(justin): figure out error handling here.
                        let msg = msg.unwrap();
                        let out = bincode::deserialize(&msg).unwrap();
                        incoming_send.send(out).await.unwrap();
                    }
                    // TODO(justin): The connection is closed, so we should
                    // clean up its metadata.
                });
            }
        });

        let (send_port, recv_port) = self.make_edge("tcp ingress handoff");
        self.add_input_from_stream("tcp ingress stream", send_port, incoming_messages.map(Some));

        (port, recv_port)
    }
}
