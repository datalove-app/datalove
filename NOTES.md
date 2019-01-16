DHT:
  - what guarantees data availability?
  - how does one prevent DHT poisoning (i.e., being served massive amounts of incorrect data)?
  
HApp development:
  - support for transactions? are bundles the same as transactions?
  - where is the JSONSchema used?
  - Errors:
    - does "throwErrors" apply to all native functions?

Research TODO:
  - figure out:
    - error throwing behaviour
    - validation behaviour
    - `call` and `send` return values and types
    - data-specific details
      - role of JSONSchema files
      - relationship between local node data and DHT data
      - how entries, links and other types of data work together and where they can be stored/how they can be retrieved
        + "Sharing" - "private" == committed to local chain, "public" == committed to local DHT
      - how `get`/`query` work
    - zome function capabilities (callable by zomes, by other apps, by web clients)
    - then, write blog post on holochain API and general architecture
  - figure out:
    - consensus patterns for coordination amongst multiple agents
    - then, write a blog post about coordination and consensus patterns in holochain
  - figure out:
    - testing/scenario building and reliability/flow
    - how to set up a multi-node env
    - dockerize the whole process
    - then, write blog post about unit + integration testing in single-node envs and integration testing multi-node environments
  - figure out key management patterns, i.e. how to:
     - revoke an app's key
     - replace an app's key
     - delegate signing for a single agent across multiple nodes:
       - https://github.com/holochain/holochain-rust/blob/develop/doc/architecture/decisions/0006-splitting-agent-into-front-house-back-house-or-not.md
     - then, write a blog post about the pattern

Development TODO:
  - finish types:
    - according to throwing behaviour

Launch TODO:
  - DPKI
    - associate app node keys with a particular user
    - allow profile information to be stored
      - allow third-party social media profile info to be referenced
    - allow App Agent keys to be revoked
    - allow App Agent keys to be replaced
      - allow it to be done via social multisig recovery 
  - data management
    - allow local chain data to be backed up
    - allow local chain data to be restored to a new device
  - App Agent Delegation
    - allow an Agent to delegate certain responsibilities to another private key 
  - Datalove
    - atomic transactions across multiple Agents
    - Agent-centric (aka vertex-centric) graph algorithms across multiple Agents
