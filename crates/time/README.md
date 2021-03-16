# datalove-time

> a verifiable vector clock for p2p networks

## Installation

```toml
[dependencies]
datalove-time = "0.0.1"
```

## Motivation

### Abstract

Blockchains, despite their controversy, have undeniably satisfied one core
use-case: global, decentralized time-keeping. This, however, comes at a
significant cost, both to every peer individually and the network as a whole -
namely, the required use of a proof system (e.g. proof of work, proof of stake,
etc) to disincentivize spamming the shared ledger.

In this repo, I implement a verifiable peer-to-peer vector clock and suggest
that in networks where total ordering of events can be avoided, this clock could
enable many of the use cases blockchains have failed to produce.

### Construction

[Bloom clocks](https://arxiv.org/abs/1905.13064) are a mashup of bloom filters
and vector clocks. This construction uses a counting bloom filter for simplicity
with the following behaviors:

- when witnessing events, behave like a bloom filter: events are hashed and the
  appropriate indices are incremented.
- when synchronizing clocks, behave like a vector clock: set each counter to the
  max of the clocks.

Like bloom filters, they can be queried probabilistically for witnessed events
with a configurable false positive rate that determines the clock's size. Like
vector clocks, partial order of witnessed events can be deduced by comparing the
two clocks.

Assuming no nodes were byzantine, a bloom clock alone would suffice for
attaining a partial order of all events in a p2p network, even in the presence
of network splits. To weaken this constraint, it is also implemented as an R1CS
gadget and executed within a [PCD]()-capable NARK system, modifying the above
behaviors to input and output a verifiable proof of correct execution.

## Implementation

// enumerate the chosen constants, curves and proof systems

## Analysis

// jepsen? something to detail how it performs IRL

In practice, clocks only need to be incremented (or synchronized) when
propagating events to (or receiving events from) outside a trust boundary.
