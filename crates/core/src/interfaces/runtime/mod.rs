//! the interface use case boundary
//! CLI, graphql (primary port adapters)
//!
//! pattern:
//!     (incoming (stdin, queries, mutations))
//!         - parses msgs if necessary
//!         - directly call methods in /runtime or /wasi, awaits response
//!         - serializes msg if necessary
//!     (outgoing (stdout, stderr, subscriptions))
//!         - on init, service subscribes to `Stream` from /runtime or /wasi
//!         - on event, service publishes to provided `Sink`s
//!
//! i.e. codegens/provisions externally-facing WASI CLI and graphql services
//! e.g. macros that:
//!     - given a WASI config
//!         - impls <Query, Mutation, Subscription>
//!         - but for the WASI instance wrapped in a core type
//!     - given a WASI config produces an impl Subcommand, that can parse CLI
//!     args and apply itself to a running WASI instance
//!     - given an Admin command list produces a CLI service and an
//!     authenticated graphql service
//!     -
