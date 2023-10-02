[an introduction to state-based crdts](https://www.bartoszsypytkowski.com/the-state-of-a-state-based-crdts)

conflict-free
- data structures that dont require exclusive write access
- detect concurrent updates and have deterministic conflict resolution (from contained metadata)

two kinds:
- state-based (convergent)
  - generally embed conflict-resolution metadata in the data structure
- operation-based (commutative)
  - generally embed conflict-resolution metadata in the associated replication protocol
- delta-state-based (derives a delta from the state, to be replicated)

consist of two parts:
- replication protocol
- state application

gcounter `Map<ActorID, Int>`:
- each actor increments its own counter
- `merge` is a union of the counters, taking the `max` of an actor's counter when it appears in each
notes:
- each replica maintains every replica's state
- no causality tracking

vector clock (aka version vector) `Map<ActorID, Int>`:
- same as gcounter
- partialord can be determined by comparing clocks for total/indeterminate order
