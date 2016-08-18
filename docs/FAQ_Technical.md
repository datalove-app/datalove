<!-- for software engineers and those in alternative money community -->

# What prevents inflation and explosive creation of credit?

# What prevents Sybil attacks?

# As a smart contract developer, what are some gotchas I should be aware of?
- users have to create an offer for a token *before* they can accept it, so if you are issuing a fixed-supply token, you should implement a `register` function that users can call before you issue them the initial tokens (see [Creating an ERC20 Token]().
- the WhuffieAPI contract implements the most basic functionality any tradeable asset would need (namely, issuing credit and sending it from one account to another), which means that if you are writing contracts on top of Whuffie, they will likely need to make use of `DELEGATECALL` to invoke WhuffieAPI functions on your users' behalf.
- there will likely be multiple active WhuffieAPI contracts in use simultaneously (and will likely only be disabled in the event of bugs being found), so your contracts may want/have to take newer API addresses into account.
