//!

use crate::dev::*;
use std::{
    boxed::Box,
    convert::TryFrom,
    fmt::Debug,
    future,
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context, Poll},
};

///
pub trait Service<C: Core>: Sized {
    type Config: Debug;
    // type State: hidden::Persistable;

    fn start(config: Self::Config) -> Self;

    fn reconfigure(&mut self, config: Self::Config) {}

    // fn snapshot(&self) -> Self::State;
}

///
pub type AsyncResponse<E> = Pin<Box<dyn future::Future<Output = Result<E, Error>>>>;

/// Base trait for queryable datalove services.
///
///
pub trait QueryHandler<C: Core, Q: Query<C> + From<C::Query>> {
    fn handle_query(&self, query: Q) -> AsyncResponse<Q::Response>;
}

/// Represents an eagerly resolvable, read-only query.
///
/// ... describe's a read-only query against a `Service`'s state.
pub trait Query<C: Core>: hidden::Persistable {
    // type Response: Into<<C::Query as Query<C, C>>::Response>;
    type Response: hidden::Persistable;

    // fn resolve(&self, state: &S::State) -> Self::Response;
}

/// Base trait for event-driven datalove services.
///
/// The API is designed to accomodate seperation of commands and their side
/// effects, as well as separate external API core use cases from their internal
/// usage inter-service.
pub trait EventHandler<C: Core, E: Event<C> + From<C::Event>> {
    /// Handles a service event.
    ///
    /// If successful, the service may persist the event, hand off the event to
    /// another service, or queue the effect to be polled later.
    fn handle_event<Ev: Into<E>>(&mut self, event: Ev) -> AsyncResponse<Option<E::Effect>>;

    /// Polls the service for outstanding effects to be performed by drivers or
    /// other services.
    fn poll_effects(&mut self, cx: &mut Context) -> Poll<E::Effect>;
}

/// Represents a lazily-evaluated service event.
///
/// ... describes a `Service`'s requirements to satisfy a `Core` use case action.
pub trait Event<C: Core>: hidden::Persistable {
    // type Effect: Effect + Into<<C::Event as Event<C, C>>::Effect>;
    type Effect: Effect;

    fn id(&self) -> &Uuid;

    /// Whether or not the event has been persisted and it's effects have been
    /// fully applied.
    fn is_completed(&self) -> bool {
        true
    }

    // /// Attempts to apply an event to the service's state, returning an
    // /// [`crate::error::Error`] if it cannot, and any residual effects to be
    // /// applied.
    // fn try_apply(&self, state: &mut S::State) -> Result<Option<Self::Effect>, Error>;
}

///
pub trait Effect: Debug {
    fn id(&self) -> &Uuid;

    /// Given a successful handling of an event, list the drivers should perform
    /// this event's effects.
    ///
    /// Since
    fn drivers(&self) -> &[DriverType];
}

#[doc(hidden)]
pub static NIL_UUID: Uuid = Uuid::nil();

// TODO: remove this
impl<C: Core> Event<C> for () {
    type Effect = ();
    fn id(&self) -> &Uuid {
        &NIL_UUID
    }
}

impl Effect for () {
    fn id(&self) -> &Uuid {
        &NIL_UUID
    }

    fn drivers(&self) -> &[DriverType] {
        &[]
    }
}

impl<E: Effect> Effect for Option<E> {
    fn id(&self) -> &Uuid {
        match self {
            None => &NIL_UUID,
            Some(e) => e.id(),
        }
    }

    fn drivers(&self) -> &[DriverType] {
        match self {
            None => &[],
            Some(e) => e.drivers(),
        }
    }
}

pub(crate) mod hidden {
    use crate::dev::*;
    use std::fmt::Debug;

    #[doc(hidden)]
    pub trait Persistable: Debug + Serialize + for<'de> Deserialize<'de> {}
    impl<T> Persistable for T where T: Debug + Serialize + for<'de> Deserialize<'de> {}
}
