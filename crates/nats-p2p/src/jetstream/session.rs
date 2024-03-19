use crate::core::{session, ClientOp, CoreMessage};
use ractor::{Actor, ActorProcessingErr, ActorRef, SupervisionEvent};

#[derive(Debug)]
pub struct Session<S: session::Split> {
    inner: InnerSession<S>,
}

impl<S: session::Split> Default for Session<S>
where
    InnerSession<S>: Default,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

type InnerSession<S> = session::Session<S, Receiver<<S as session::Split>::Read>>;
type InnerSessionState<S> = <InnerSession<S> as Actor>::State;
type InnerSessionArgs<S> = <InnerSession<S> as Actor>::Arguments;

impl<S: session::Split> Actor for Session<S>
where
    InnerSession<S>: Actor<Msg = CoreMessage>,
{
    type Msg = CoreMessage;
    type State = InnerSessionState<S>;
    type Arguments = InnerSessionArgs<S>;

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        self.inner.pre_start(myself, args).await
    }

    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        self.inner.post_start(myself, state).await
    }

    async fn post_stop(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        self.inner.post_stop(myself, state).await
    }

    async fn handle_supervisor_evt(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: SupervisionEvent,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        self.inner.handle_supervisor_evt(myself, msg, state).await
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        self.inner.handle(_myself, msg, state).await
    }
}

#[derive(Debug, Default)]
pub struct Receiver<R> {
    inner: InnerReceiver<R>,
}

type InnerReceiver<R> = session::Receiver<R>;
type InnerReceiverState<R> = <InnerReceiver<R> as Actor>::State;
type InnerReceiverArgs<R> = <InnerReceiver<R> as Actor>::Arguments;

impl<R> Actor for Receiver<R>
where
    InnerReceiver<R>: Actor<Msg = ClientOp>,
{
    type Msg = ClientOp;
    type State = InnerReceiverState<R>;
    type Arguments = InnerReceiverArgs<R>;

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        self.inner.pre_start(myself, args).await
    }

    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        self.inner.post_start(myself, state).await
    }

    async fn post_stop(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        self.inner.post_stop(myself, state).await
    }

    async fn handle_supervisor_evt(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: SupervisionEvent,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        self.inner.handle_supervisor_evt(myself, msg, state).await
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            ClientOp::Publish { .. } => Ok(()),
            ClientOp::Subscribe { .. } => Ok(()),
            ClientOp::Unsubscribe { .. } => Ok(()),
            _ => self.inner.handle(_myself, msg, state).await,
        }
    }
}
