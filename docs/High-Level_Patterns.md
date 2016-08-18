# How to execute a single-buy or sell Offer
- make sure you possess the desired amount of credit you want to sell
- make sure you can accept as much credit as you're wanting to buy
- call `trade`, passing in the addresses of the credits you're buying/selling and receiving
- if successful, `freeze` the credit you just received
- for partial buys, to be completed in chunks:
  - `trade` and `freeze` whatever you credit you can at once
  - schedule another partial buy (`trade` and `freeze`) for the remainder, until order is satisfied

# How to issue your own Credit on Whuffie

# How to Create an ERC20-compliant token on Whuffie

# How to use an existing ERC20-compliant token on Whuffie
- use the Whuffie Mirror Market contract to use an existing (or create) a mirrored-version of the token!
å