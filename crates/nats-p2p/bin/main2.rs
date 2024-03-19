#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use nats_p2p::*;
use futures::{Sink, Stream, StreamExt};
use hydroflow::{
    hydroflow_syntax,
    scheduled::{graph::Hydroflow, graph_ext::GraphExt, handoff::VecHandoff},
};
use tokio::{
    io::{self, AsyncRead, AsyncWrite},
    net::TcpListener,
};
use tokio_stream::wrappers::TcpListenerStream;
use tokio_util::codec::{FramedRead, FramedWrite};
use tracing_subscriber::{prelude::*, EnvFilter};
struct Args {}
fn inbound_stream<R: AsyncRead>(
    reader: R,
) -> impl Stream<Item = Result<ClientOp, Error>> {
    FramedRead::<_, Codec<ClientOp>>::new(reader, Default::default())
}
fn outbound_sink<W: AsyncWrite>(writer: W) -> impl Sink<ServerOp, Error = Error> {
    FramedWrite::<_, Codec<ServerOp>>::new(writer, Default::default())
}
fn session<'a>(io: impl NetworkSplit) -> Hydroflow<'a> {
    let mut df = {
        #[allow(unused_qualifications)]
        {
            use ::hydroflow::{var_expr, var_args};
            let mut df = ::hydroflow::scheduled::graph::Hydroflow::new();
            df.__assign_meta_graph(
                "{\"nodes\":[{\"value\":null,\"version\":0}],\"edge_types\":[{\"value\":null,\"version\":0}],\"graph\":[{\"value\":null,\"version\":0}],\"ports\":[{\"value\":null,\"version\":0}],\"node_subgraph\":[{\"value\":null,\"version\":0}],\"subgraph_nodes\":[{\"value\":null,\"version\":0}],\"subgraph_stratum\":[{\"value\":null,\"version\":0}],\"node_varnames\":[{\"value\":null,\"version\":0}],\"flow_props\":[{\"value\":null,\"version\":0}],\"subgraph_laziness\":[{\"value\":null,\"version\":0}]}",
            );
            df.__assign_diagnostics("[]");
            df
        }
    };
    let (reader, writer) = io.split();
    let reader = FramedRead::<_, Codec<ClientOp>>::new(reader, Default::default())
        .map(|res| res.ok());
    let (send_port, recv_port) = df
        .make_edge::<_, VecHandoff<ClientOp>>("tcp ingress handoff");
    df.add_input_from_stream("tcp stream", send_port, reader);
    df
}
async fn server(config: Config) -> Result<Hydroflow<'static>, Error> {
    let listener = TcpListenerStream::new(
        TcpListener::bind(config.listen_addr()).await?,
    );
    Ok({
        #[allow(unused_qualifications)]
        {
            use ::hydroflow::{var_expr, var_args};
            let mut df = ::hydroflow::scheduled::graph::Hydroflow::new();
            df.__assign_meta_graph(
                "{\"nodes\":[{\"value\":null,\"version\":0},{\"value\":{\"Operator\":\"source_stream(listener)\"},\"version\":1},{\"value\":{\"Operator\":\"map(Result :: unwrap)\"},\"version\":1},{\"value\":{\"Operator\":\"map(| stream | (stream.local_addr().unwrap(), stream))\"},\"version\":1},{\"value\":{\"Operator\":\"for_each(| (addr, _) | println! (\\\"opened connection: {}\\\", addr))\"},\"version\":1}],\"edge_types\":[{\"value\":null,\"version\":0},{\"value\":\"Value\",\"version\":1},{\"value\":\"Value\",\"version\":1},{\"value\":\"Value\",\"version\":1}],\"graph\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":2,\"version\":1},{\"idx\":3,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1}],\"ports\":[{\"value\":null,\"version\":0},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1}],\"node_subgraph\":[{\"value\":null,\"version\":0},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1}],\"subgraph_nodes\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1},{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1}],\"subgraph_stratum\":[{\"value\":null,\"version\":0},{\"value\":0,\"version\":1}],\"node_varnames\":[{\"value\":null,\"version\":0},{\"value\":\"listeners\",\"version\":1},{\"value\":\"listeners\",\"version\":1},{\"value\":\"listeners\",\"version\":1},{\"value\":\"sessions\",\"version\":1}],\"flow_props\":[{\"value\":null,\"version\":0}],\"subgraph_laziness\":[{\"value\":null,\"version\":0}]}",
            );
            df.__assign_diagnostics("[]");
            let mut sg_1v1_node_1v1_stream = {
                #[inline(always)]
                fn check_stream<
                    Stream: ::hydroflow::futures::stream::Stream<Item = Item>
                        + ::std::marker::Unpin,
                    Item,
                >(
                    stream: Stream,
                ) -> impl ::hydroflow::futures::stream::Stream<
                    Item = Item,
                > + ::std::marker::Unpin {
                    stream
                }
                check_stream(listener)
            };
            df.add_subgraph_stratified(
                "Subgraph GraphSubgraphId(1v1)",
                0,
                (),
                (),
                false,
                move |context, (), ()| {
                    let op_1v1 = std::iter::from_fn(|| {
                        match ::hydroflow::futures::stream::Stream::poll_next(
                            ::std::pin::Pin::new(&mut sg_1v1_node_1v1_stream),
                            &mut std::task::Context::from_waker(&context.waker()),
                        ) {
                            std::task::Poll::Ready(maybe) => maybe,
                            std::task::Poll::Pending => None,
                        }
                    });
                    let op_1v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_1v1__source_stream__loc_unknown_start_0_0_end_0_0<
                            Item,
                            Input: ::std::iter::Iterator<Item = Item>,
                        >(input: Input) -> impl ::std::iter::Iterator<Item = Item> {
                            struct Pull<
                                Item,
                                Input: ::std::iter::Iterator<Item = Item>,
                            > {
                                inner: Input,
                            }
                            impl<
                                Item,
                                Input: ::std::iter::Iterator<Item = Item>,
                            > Iterator for Pull<Item, Input> {
                                type Item = Item;
                                #[inline(always)]
                                fn next(&mut self) -> Option<Self::Item> {
                                    self.inner.next()
                                }
                                #[inline(always)]
                                fn size_hint(&self) -> (usize, Option<usize>) {
                                    self.inner.size_hint()
                                }
                            }
                            Pull { inner: input }
                        }
                        op_1v1__source_stream__loc_unknown_start_0_0_end_0_0(op_1v1)
                    };
                    let op_2v1 = op_1v1.map(Result::unwrap);
                    let op_2v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_2v1__map__loc_unknown_start_0_0_end_0_0<
                            Item,
                            Input: ::std::iter::Iterator<Item = Item>,
                        >(input: Input) -> impl ::std::iter::Iterator<Item = Item> {
                            struct Pull<
                                Item,
                                Input: ::std::iter::Iterator<Item = Item>,
                            > {
                                inner: Input,
                            }
                            impl<
                                Item,
                                Input: ::std::iter::Iterator<Item = Item>,
                            > Iterator for Pull<Item, Input> {
                                type Item = Item;
                                #[inline(always)]
                                fn next(&mut self) -> Option<Self::Item> {
                                    self.inner.next()
                                }
                                #[inline(always)]
                                fn size_hint(&self) -> (usize, Option<usize>) {
                                    self.inner.size_hint()
                                }
                            }
                            Pull { inner: input }
                        }
                        op_2v1__map__loc_unknown_start_0_0_end_0_0(op_2v1)
                    };
                    let op_3v1 = op_2v1
                        .map(|stream| (stream.local_addr().unwrap(), stream));
                    let op_3v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_3v1__map__loc_unknown_start_0_0_end_0_0<
                            Item,
                            Input: ::std::iter::Iterator<Item = Item>,
                        >(input: Input) -> impl ::std::iter::Iterator<Item = Item> {
                            struct Pull<
                                Item,
                                Input: ::std::iter::Iterator<Item = Item>,
                            > {
                                inner: Input,
                            }
                            impl<
                                Item,
                                Input: ::std::iter::Iterator<Item = Item>,
                            > Iterator for Pull<Item, Input> {
                                type Item = Item;
                                #[inline(always)]
                                fn next(&mut self) -> Option<Self::Item> {
                                    self.inner.next()
                                }
                                #[inline(always)]
                                fn size_hint(&self) -> (usize, Option<usize>) {
                                    self.inner.size_hint()
                                }
                            }
                            Pull { inner: input }
                        }
                        op_3v1__map__loc_unknown_start_0_0_end_0_0(op_3v1)
                    };
                    let op_4v1 = ::hydroflow::pusherator::for_each::ForEach::new(|
                        (addr, _)|
                    {
                        ::std::io::_print(
                            format_args!("opened connection: {0}\n", addr),
                        );
                    });
                    let op_4v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_4v1__for_each__loc_unknown_start_0_0_end_0_0<
                            Item,
                            Input: ::hydroflow::pusherator::Pusherator<Item = Item>,
                        >(
                            input: Input,
                        ) -> impl ::hydroflow::pusherator::Pusherator<Item = Item> {
                            struct Push<
                                Item,
                                Input: ::hydroflow::pusherator::Pusherator<Item = Item>,
                            > {
                                inner: Input,
                            }
                            impl<
                                Item,
                                Input: ::hydroflow::pusherator::Pusherator<Item = Item>,
                            > ::hydroflow::pusherator::Pusherator for Push<Item, Input> {
                                type Item = Item;
                                #[inline(always)]
                                fn give(&mut self, item: Self::Item) {
                                    self.inner.give(item)
                                }
                            }
                            Push { inner: input }
                        }
                        op_4v1__for_each__loc_unknown_start_0_0_end_0_0(op_4v1)
                    };
                    #[inline(always)]
                    fn check_pivot_run<
                        Pull: ::std::iter::Iterator<Item = Item>,
                        Push: ::hydroflow::pusherator::Pusherator<Item = Item>,
                        Item,
                    >(pull: Pull, push: Push) {
                        ::hydroflow::pusherator::pivot::Pivot::new(pull, push).run();
                    }
                    check_pivot_run(op_3v1, op_4v1);
                },
            );
            df
        }
    })
}
fn main() -> io::Result<()> {
    let body = async {
        let fmt = tracing_subscriber::fmt::layer().with_target(false).with_ansi(true);
        tracing_subscriber::registry()
            .with(fmt)
            .with(EnvFilter::from_default_env())
            .init();
        let config = Config::default();
        let _ = server(config)
            .await?
            .run_async()
            .await
            .ok_or_else(|| io::Error::other("server error"))?;
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
