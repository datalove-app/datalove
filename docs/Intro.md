# Introduction to Whuffie

Whuffie is a cryptocredit network and API designed to allowing you to easily issue and seamlessly exchange your own custom currencies, credits and tokens.

The contract system accomplishes this by exposing two basic, but incredibly powerful actions:
- you can issue their own custom currencies, tokens and credits denominated in any unit-of-account of your choosing (hereby collectively referred to as "credits"), or you can import existing balances from other Ethereum ERC20 contracts
- you can "share" with other users some amount of the credits you hold (and credits shared with you by other users) at any exchange rate you choose, essentially creating an open offer to hold and exchange your credits for another's

Because Whuffie implements the minimum subset of features required for all currencies, credits and tokens (namely: issuance, transfer and exchange), *Whuffie can be used to implement any kind of currency, token or credit*, such as:
- crowdsale, ICO and DAO tokens
- social network and reputation currencies
- charity and volunteering incentive systems
- in-game credits
- rewards/loyalty points systems
- a full-blown currency/commodity exchange
- mutual credit and community currencies (both centralized and fully peer-to-peer)
- even localized basic income systems...

... and because we've included a path-finding algorithm in the contract to traverse over everyone's open offers, any of your credits can now be sent from one user to another, **even if the recipient doesn't accept the credit you're sending**.

# Why issue credits on Whuffie?

Using Whuffie for launching your credit comes with innumerable benefits, for example:
- In general, the value of any credit is derived from what it can be traded for. However, creating a robust-enough community around a credit such that it can be used to pay for goods, services and wages is an incredibly difficult endeavour (think about how long it took Bitcoin to be exchanged for pizza and alpaca socks, and all the other altcoins that failed to get that level of traction).
  - With Whuffie, every credit you issue is now connected to most every other credit on the  Whuffie network, through the offers users set to accept yours and other people's credits
- Every token or credit issued by a single authority is suceptible to individuals "gaming the system" and Sybil attacks (see the Stellar issuance [debacle](https://news.ycombinator.com/item?id=8126282)).
  - With Whuffie, your token (for example, "REP") can be owned and issued by your users, making someone's REP balance and spendability be a function of the acceptance of that user's REP throughout the greater network of all REP tokens (similar to using links between webpages to calculate PageRank or using PGP key signing to create a web-of-trust).

# How is this possible?

The Whuffie engine maintains a [graph]() of all offers, which lets users define whose credit they want to accept, the maximum amount they'll want to accept (which can be increased or decreased at a later time), and at what exchange rate they'll accept it (between the credit they issue and the credit they want).

By analyzing the graph of credits and offers in real time, two people who do not hold each other's credit can now trade with each other through intermediary offers, taking as few or as many "hops" thru offers as necessary, at the lowest cost possible. This is easiest to explain by example:

- Say I have Bob's USD, and the shoemaker only accepts Bob's CNY.
- I'll ask to send 50 USD for the shoes, and the smart contract code will transparently and automatically:
  1. send 50 USD to Bob
  2. Bob receives 50 USD, then sends 50 USD worth of CNY to the shoemaker
- Notice that at the end of this transaction, my USD balance went down, the shoemaker's CNY balance went up, while Bob's USD balance increased and his CNY balance decreased, possibly making money along the way depending on the exchange rate he established between USD and CNY

## So why is this a big deal?

Because users are free to decide whose credit they want to accept (and thus, give value), everyone is now free to issue any kind of credit they want, knowing that the more the network finds it valuable, the more acceptable and fungible it will be among greater parts of the cryptocredit network.

So whether you're trying to create a currency for [a social network](), a token backed by [wind energy production in Romania](), or even a [basic income system](), the Whuffie engine will give your users access to a greater network of credits (and goods and services) than just your own credit network, giving your credits and your users *greater local significance and greater global impact simultaneously*.

<!--
Too good to be true, right? To explain how all of this is possible, I'll need to provide a bit of backstory:

In the beginning of human civilization, trade was conducted not by barter, but by trust and credit - people traded their wares today to the people they trusted only to receive something back in the future
(EX: baskets for chickens).
This was possible b/c people knew each other their entire lives and could trust that they wouldn't leave town, rendering any earned credit but unspent worthless.

The problem was that this didn't scale as easily as currencies run by the state and backed by commodities like gold
?????????????????????????????????
for one, dunbar's number suggests that it is impossible for any one person in modern society to maintain enough close relationships to be able to rely solely on p2p credit for trade.
(EX: go into how it could be difficult to use the credit you have with the chicken farmer on blacksmith, MAYBE WATCH MONEY IS DEBT)
Also, at the time, the logistics for tabulating every citizen's balances were too high. As a result,  took over, and the rest is history.
?????????????????????????????????

Today however, with modern CPUs and smart contracts, we can trustlessly solve the second problem of managing p2p balances, which then inadvertantly solves the first problem of limited trade.
-->