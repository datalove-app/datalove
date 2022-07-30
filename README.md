<!-- [Whuffie]() — the social cryptocurrency
==================================================

What is Whuffie?
----------------
Whuffie is an attempt at creating a money system that truly captures the essence of money:
> ... in the old days, if you were broke but respected, you wouldn't starve; contrariwise, if you were rich and hated, no sum could buy you security and peace. By measuring the thing that money really represented — your personal capital with your friends and neighbors — you more accurately gauged your success.

> \- Cory Doctorow, *Down and Out in The Magic Kingdom*

Note:
-----
Whuffie is currently an ugly, pre-alpha WIP to be seen and used by no one.

Questions?
----------

If you have any questions, please feel free to email me. Thanks for checking this out!
 -->

datalove
========

an experiment in rebooting the web

features:
  - use powerful Rust macros to generate complex distributed services that:
    - speak GraphQL
    - are compiled to WASI
    - send and receive all traffic over a p2p, end-to-end encrypted mesh network
  - run any single-threaded WASI-compiled binary as a p2p app with declarative
   YAML configuration
    - allow some services to run as singletons, replicas or clusters, on selected
     devices or everyones (if they allow, of course)
    - require that some services have additional authentication with certain
     scopes, automatically wrapping each request and response
  - app installation, account/device provisioning and recovery, data
   replication and more can be handled via CLI or desktop/mobile app
    - in-app web browser serves apps that can call any number of
   running services, with configurable permissions

design philosophy:
  - user-centric:
    from zk rollups of user keys, to social account recovery, to developing mobile and desktop apps (and even headless raspberry pi firmware!) with an integrated web browser, everything was designed from the user-up
  - no tokens, no mining, no treasuries. just pure, p2p credit
    earn point-to-point credit for providing public services, like replicating data. but b/c credit is transitive, you can use multiple points to pay strangers for private services (writes to objects, transactions)
  - forward-thinking or backwards-compatible apps that are p2p by default** (?? how??)
    apps written in WASM are by default sandboxed and...


datalove builds on a few projects and experimental cryptography libraries listed
 below.

    - [`yggdrasil-go`]()
    - [`GraphQL`]() via [`juniper`]()
    - [`IPLD`]() + [`CBOR`]() via [`lipipld`]()
    - [`IPLD Schemas and Representations`]() via [`libipld-schema`]()
    - ... crypto

as a result of datalove's architecture, you'll also find some familiar features
/services from popular tools:

| [IPFS Bitswap]()      |
| [docker containers]() | [`WASM`]() via [`wasmtime`]() + [`lucetc`]()
| [docker-runtime]()    | [`datalove` daemon]() via [`WASI`]() + [`lucet-runtime`]()
| kubernetes
| istio
| ...
