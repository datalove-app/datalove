# Introduction to Whuffie

Whuffie is a social cryptocredit engine.
- users create their own currencies or credits
- users individually choose to accept other's credits at some user-defined exchange rate
- credits can be sent from one user to another if there is a path of acceptance between them
- ?? a user's reputation can be measured by their credit's acceptance throughout the graph, similar to PageRank and linking URLs

B/c of these rules, any currency or token that can be issued and sent between accounts can be implemented on top of Whuffie, such as:
- social network and reputation currencies
- rewards points systems
- video game credits/tokens
- community currencies & mutual credit systems
- charity and volunteering incentive systems
- basic income credit systems

Too good to be true, right? To explain how all of this is possible, I'll need to provide a bit of backstory:

In the beginning of human civilization, trade was conducted not by barter, but by trust and credit - people traded their wares today to the people they trusted only to receive something back in the future
(EX: baskets for chickens).
This was possible b/c people knew each other their entire lives and could trust that they wouldn't leave town, rendering any earned credit but unspent worthless.

The problem was that this did't scale as easily as currencies run by the state and backed by commodities like gold
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
