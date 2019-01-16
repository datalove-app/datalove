## responsibilities:
<!-- https://github.com/blockstack/blockstack-proofs-py/blob/master/blockstack_proofs/htmlparsing.py -->

### integration behaviours:

#### general methods:

- [ ] `service.getToken()`

#### creating service <-> DLT relationships:

- [ ] `service.linkAccounts`
  - either set/updates profile text, or creates post/gist/etc

#### verifying service <-> DLT relationships:

- [ ] `service.getAddressFromUsername(stellarSDK, userId) => Promise<address>`
  - create the proof URL from userId
  - fetch the proof
  - extract the address from the proof
  - query horizon for the account's named entry for the service
  - verify the named entry userId matches the one we started with
- [ ] `service.getAddressFromProofURL(stellarSDK, proofURL) => Promise<address>`
  - fetch the proof
  - extract the address from the proof
  - query horizon for the account's named entry for the service
  - verify the named entry proof URL matches the one we started with

<!-- - [ ] `service.getProof(address) => Promise<proofId>` -->
<!-- - [ ] `getUsernameFromProof(proofId) => Promise<>` -->
<!-- - [ ] `service.getUsernameForAddress(address) => Promise<username>` -->
<!-- - [ ] `service.validateProof(address, username) => Promise<boolean>` -->


### injectable page behaviours:

- [ ] `setup`:
  - establishes all event handlers for the page
  - searches cache for all verified usernames
  - renders any UI on top of the page, i.e.:
    - highlights connected usernames
    - amount shared on a comment
    -

- [ ] `onShare(userId, metadata)`
  - sends message to background process
  - should be called when attempting to share credit with a user
  - userId is used to fetch the user's stellar address
- [ ] `onUnshare(userId, metadata)`
- [ ] ``