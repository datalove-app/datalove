[Whuffie](https://github.com/sunny-g/whuffie)
================================================
<!--
[![Dependency Status](https://david-dm.org/sunny-g/whuffie.svg?path=frontend)](https://david-dm.org/sunny-g/whuffie?path=native)
[![devDependency Status](https://david-dm.org/sunny-g/whuffie/dev-status.svg?path=frontend)](https://david-dm.org/sunny-g/whuffie?path=native#info=devDependencies)
[![Join the chat at https://gitter.im/sunny-g/whuffie](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/sunny-g/whuffie)
-->

A cryptocredit network and API, built on Ethereum: issue and seamlessly exchange custom p2p credits (or any other arbitrary currencies, credits or tokens).

About
-----

Whuffie is an attempt at creating a money system that truly captures the essence of money:
> ... in the old days, if you were broke but respected, you wouldn't starve; contrariwise, if you were rich and hated, no sum could buy you security and peace. By measuring the thing that money really represented — your personal capital with your friends and neighbors — you more accurately gauged your success.

> \- Cory Doctorow, *Down and Out in The Magic Kingdom*

More specifically, because Whuffie implements the minimum subset of features required for all currencies, credits and tokens (i.e. issuance, transfer, and exchange), you can use it to **implement any currency, token, credit** or **higher-level exchange platform**, such as:

- crowdsale, ICO and DAO tokens
- in-game credits
- rewards/loyalty point systems
- social network and reputation currencies
- charity and volunteering incentive systems
- a full-blown currency/commodity exchange
- mutual credit and community currencies (with or without demurrage)
- a low-level, p2p replacement for ACH, SWIFT, MasterCard *and* Visa
- even custom basic income systems...

... and because we've included a path-finding algorithm in the contract to traverse over everyone's open offers, any of your credits can now be sent from one user to another, **even if the recipient doesn't accept the credit you're sending**.

Components
----------

- `contracts/` - Ethereum contracts in Solidity
- `docs/` - Documentation about theory and implementation
- `native/` - React Native frontend (currently, not in active development)

### Note:

Whuffie is currently in development, experimental and untested.

### Questions?

If you have any questions, please feel free to email me. Thanks for checking this out!
