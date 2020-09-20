# notes

## Multi-device User requirements

- each device has:
  - an eddsa keypair
  - a rank

- each txn must:
  - have a type/struct
  - ? be signed by a "quorum" (min rank sum) of device keypairs

## Root notes:

what is a root on different platforms?
- if mobile/macbook:
  - enclave decrypts
- if desktop/server (aka stm32xxxx):
  -

## The Entire Proof System characteristics

- initial proof:
  - private inputs:
    1.
    2. ... signed txn(s)
  - public input:
    1. initial clock, including the user init tx
    2. ? ... new keytree hash (with tx (batch))
    3. ? ... tx (or merkle tree batch of tx) hash
      why?
  - proves:
    - TODO:
    - about the keytree:
      - "adds" key inserted into the empty merkle tree
      - signed by that key
    - about the clock:
      - was produced via "valid" txns

- every subsequent proof:
  - private inputs:
    1.
    2. ... signed txn(s)
      all are signed by keys in the keytree
      that they are valid as part of the keytree + clock + txn proof system
  - public input:
    1. current clock
    2. ?
    3. ? ... tx (or merkle tree batch of tx?) hash
      why?
  - proves:
    - TODO:
    - about the keytree
    - about the clock:
      - every new clock by definition contains every other in the past

## User Stories

- in a rollup:
  - ppl submit txns to update their state
    - this implies they need to be subscribed to state changes (but not leaves)
    -

- IPNS alt
  - given a hash/did:
    - proof should prove
