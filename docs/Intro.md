# Introduction to Whuffie

Whuffie is a social cryptocurrency network and API built on two basic, but powerful actions:
- users can issue their own custom credits and tokens (or import existing credit/token balances from other Ethereum ERC20 contracts), and can denominate them in any unit-of-account of their choosing
- users can "share" with other users some amount of the credit/tokens they hold (and credit/tokens shared with them by other users) at any exchange rate the user chooses, essentially creating an open offer to hold and exchange their credit/tokens for another's

Because Whuffie implements the minimum subset of features that all currencies, credits and tokens need, *Whuffie can be use to implement any kind of credit, token and currency*, such as:
- crowdsale, ICO and DAO tokens
- social network and reputation currencies
- a full-blown currency/commodity exchange
- charity and volunteering incentive systems
- video game credits/tokens
- rewards/loyalty points systems
- mutual credit and community currencies
- even localized basic income credit systems

... and because we've included a path-finding algorithm in the contract to traverse open offers, any of these credits and tokens can now be sent from one user to another, **even if the recipient doesn't direcly accept the credit/token being sent**


This also means that a credit/token's value (and to a certain extent, the reputation of it's issuer) can be measured by measuring that credit/token's acceptance and fungibility throughout the network, similar to using links between webpages to calculate PageRank.

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

Today however, with modern CPUs and smart contracts, we can trustlessly solve the second problem of managing p2p balances, which then inadvertantly solves the first problem of limited trade. Whuffie maintains a graph of all credit pairs users have established (called ledgers), letting users define whose credit they want, how many they want, and at what exchange rate they'll accept it.

By analyzing the graph of ledgers in real time, two people who do not hold each other's credit can now trade with each other through other user's ledgers, taking as few or as many hops as necessary at the lowest cost possible. This is easiest to explain by example:

say I have USD, and the shoemaker wants to receive CNY. I'll ask to send 50 USD, and the smart contract code will automatically:
1) send 50 USD to Bob
2) Bob receives 50 USD, then sends 50 USD worth of CNY to the shoemaker
notice that my balance went down, the shoemaker's went up, and Bob's stayed the same or increased, depending on the exchange rate he established

## So what?

Because users are free decide whose currency they want to accept (and thus, give value), everyone is now free to issue any kind of currency they want, knowing that the more the network finds it valuable, the more receivable it will be among greater parts of the network. This means that whether you're trying to create a currency for [a social network](), a token backed by [wind energy production in Romania](), or even a [basic income system](), the Whuffie engine will give your users access to a greater network of currencies than just your own, giving your currency local significance *and* global impact.
