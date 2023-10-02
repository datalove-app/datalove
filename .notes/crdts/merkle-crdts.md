[merkle-crdts](https://hector.link/presentations/merkle-crdts/merkle-crdts.pdf)

vector clocks (version vector) `Map<ActorId, Int>`:
- "compact because ... [sic] they're merely a number indicating how long the history is for every replica"

merkle-clock:
- a hash-chain of nodes, each pointing to the previous node
- tracks causality by linking to preceding node
  - if heads (CIDs) are equal, chains are equal
  - if one chain contains the other's nodes, it is totally "greater" than the other
    - if not totally ordered, a `merge` is (or can be) conducted by appending a new node to linking to both roots
    - else, the "lesser" chain is already merged into the "greater" one
  - a `fork` occurs when two different nodes link to the same preceding node

merkle-crdt: merkle-clock whose nodes contain a CRDT payload
- op-based: each node is a (commutative) op
  - requires in-order, at-most once delivery (which is already guaranteed)
  - state is produced from applying all ops
- state-based: each node is the state
- delta-based: each node is a state-delta

limitations (linear scaling with dag size):
- (state) state growth (space)
- (op, delta) state reproduction growth (time)
- (op, delta) sync growth (time)
