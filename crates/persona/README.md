## Motivation

- users should not have to deal directly with key material
- users own multiple devices and will rotate them over their identities lifetime
- users have an "age", denoted by a lamport clock/timestamp
  - this allows for ordering updates to their identity (i.e. keyset)
- users will want to require different thresholds (policies) for different applications
  - this key mgmt app requires a high threshold for add/remove/rotate/weight-change ops
  - other apps can define/verify user definitions of thresholds required for valid interactions (credit may require less or more)

### Algorithm

#### recursive proof usage

the reason we use recursive proofs is to directly and immediately tie a static identifier to the latest state

each proof proves:
-

the entire payload:
- proof (or proof cid)
- multihash of new keyset
- ? timestamp
- ? persona id (might just be implied by request for payload)
- ? identity metadata cid
- ...

proof structure:

- general structure:
  - public inputs
    - new timestamp
    - persona id
    - digest of new keyset
    - ? current/new metadata digest?
  - witnesses
    - (?signed) operation and its params (and current/new metadata digest?)
    - current keyset
    - ...

- if genesis:
  - public inputs
    - ...
  - witnesses
    - init operation (max_u8 weight + pubkey + signature)
      - verifies signature
      - verifies digest of pubkey + random commitment == persona id

- public inputs
  - ...
- witnesses
  - existing keys and weights ([u8 + pubkey])
  - group signature (subgroupid + signature(op))
  - operation
    - timestamp merge (same params as a regular proof)
      - verifies foreign proof

    TODO: which of these should just be new group joins?
    - key add (key + weight)
      - verifies group join that includes all
    - key remove
    - key weight change (u8 + pubkey)
    - ? key rotate

<!--

core behaviors...

- nodes initialize a persona with an ED25519 keypair (id is digest of ...)
  - put them in a sparse merkle tree?
  - user produces a proof that they own the key and that it has the initial weight
- single keys can be added to the tree
  - each is assigned a weight
  - a new recursive proof is produced verifying that:
    - key addition weight threshold has been exceeded
    - time is greater than the last proof

elsewhere, after adding networking and storage...

- initial identifiers can be mapped to current keypair/keytrees
  - updating the stored record requires verifying the proofs that time has passed (since last record) and that key weight thresholds were correctly exceeded
  - conflicts can be avoided by selecting
-->
