envs:

- bare-metal:
  - nixos ...
  - ape
- sudo:
- user:
- web:

deps:

- [lean4]():
  - libs, stage0 + rest
- [lurk-rs]():
  -
- [lurk.lean]:
  - parse/eval lurk
- [yatima.lean]():
  - lean4 -> (cid, yir) -> lurk
  - (cid, yir) in lean4
  - => load random cid -> yir
- [wasm.lean]():
  - ++ WAST -> lean4 -> .wasm
  - =>

feature list:

build list:

- executor/runtime/daemon
  - check caps
  - stdlib + externs (id'ed by hash):
    - `unison.lean`
      - ? abilities?
    - ``
  - handle src code (.lean files)
    - `wasm.call("leanvm.eval", src)`: evaluates the file w/ lean4.wasm
    - `lurk::prove(src -> (cid, yir) -> lurk) -> store(cid, yir)`:
- [ucm]-like devtool
  - auto-check in editor
  -
  - doc generator/explorer
  - caches:
    - ? lean4 source files
    - ? cid(yir), ? cid(lurk)
- utils
  - [arxiver]: math reader, prover and publisher
    - (eventually...) renderer, editor, notebook, lab
  - ["legacy"-compat]: dists to bridge $(program) to daemon
    - ssh
    - git
      - local: did as signer
        - ... hosts cached repo snapshots
      - remote: did as remote host,
        - ... issues/PRs
    - nix
      -
    - CA cert logs
    - dns

ROADMAP (Reverse):

stage0: distribute ape-able binary

- binary reqs/components:
  - ocaps
  - (prolly proprietary) drivers (gpu, network, fs)
  - runtime:
    - pinecone (+ bootstrap peers)
    - "bootloader" fn: uses network to fetch ...
      - the most recent datalove persona proof (metadata contains a link to "repo")
        - by hash? by DID? by petname?
        - hard-coded?
      - repo contains "recent" root module (CID(Yir) of main runtime/env)
    - wasmtime
    - ? universal webgpu?
- hosted URL:
  - binary signed by domain cert
  - binary signed by persona (persona proof links to "repo")

stage1: linking personas to repos (or arbitrary mutable state)

- repo reqs:
  - `(List (Expr, Caps), DID, time_bounds) -> List (proof+cid, block)`
    API for indexing a DID
    - provable sequential lookup-and-parse
    - ? store?

stage2: personas

- guardian is a keypair (? + merkle-log?) or another persona
- persona ops are signed by guardian + ...:
  - tx expression
  - proof of correct time state-machine application
    - for keypairs, this is a merkle-log proof tied to last persona proof sig
    - for personas, this is verification-proof of their persona since last persona proof sig

stage3: proofs

- every proof MUST share a "anchored universe" with the verifier
  - every anchor MUST be tied to a previous proof OR "zooko" info (from some other name system)
  - broke: metadata CID within persona proof
  - woke: minimal universe + function for upgrading  universe
