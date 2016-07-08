[Nanex](https://github.com/sunny-g/nanex)
-----------------------------------------
<!--
[![Dependency Status](https://david-dm.org/sunny-g/whuffie.svg?path=frontend)](https://david-dm.org/sunny-g/whuffie?path=native)
[![devDependency Status](https://david-dm.org/sunny-g/whuffie/dev-status.svg?path=frontend)](https://david-dm.org/sunny-g/whuffie?path=native#info=devDependencies)
[![Join the chat at https://gitter.im/sunny-g/whuffie](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/sunny-g/whuffie)
-->

A crypto-asset network, liquid exchange and API, built on Ethereum and Solidity.

About
-----

With Nanex, you can:

- issue, send, and exchange **any custom crypto-assets, currencies, credits, or tokens you can think of**
- migrate your **existing ERC-20 token balances** into and out of the Nanex network (coming soon)
- **build your own higher-level platforms** (like currency/commodity exchanges, social networks, mutual credit systems, basic income systems, etc) using our low-level Nanex API.

However, what makes Nanex truly unique is the ability to traverse the entire network of trades between assets whenever you want to send or exchange assets, using more than one trade if necessary.

In other words, **any of your assets can be sent** from one user to another, **even if the recipient doesn't accept the asset you're sending**.

<!-- ### Our Vision -->

Components
----------

- `contracts/` - Ethereum contracts in Solidity
- `docs/` - Documentation about theory and implementation
- `native/` - React Native frontend (currently, not in active development)

Note:
-----

:warning: Nanex is currently in active development, experimental and untested. :warning:

Questions?
----------

If you have any questions, please feel free to email me. Thanks for checking this out!


<!--
About
-----

Nanex implements the minimum set of features required for all crypto-assets (i.e. issuance, transference, and exchange). 
As a result, you can use it to **create any kind of asset, currency, token, or credit system you want** (or migrate your balances of existing Ethereum ERC-20 tokens into and out Nanex).
With our low-level crypto-asset API, you can 

A few examples of what can be built with or on top of Nanex include:

- [crowdsale](), ICO and DAO tokens                                             [](implement a simple deposit-to-vault and withdraw-from-vault functions)
- synthetic and decentralized assets, backed by assets IRL
- [social network and reputation currencies]()                                  [](implement a public credit symbol, gifting, sharing and sending operations)
- a liquid [currency/commodity exchange]() with traditional trading operations (put, call, margin, limit and stop offers, etc)
- basic income, mutual credit and community currency systems (with or without demurrage)
- gateways to existing financial institutions
- or anything else that needs a flexible yet robust accounting system...

... and because we've included a path-finding algorithm to traverse over everyone's offers to exchange assets, **any of your assets can now be transferred** from one user to another, **even if the recipient doesn't accept the asset you're sending**.
-->