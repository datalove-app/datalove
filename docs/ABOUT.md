# Here's how whuffie works:

Whuffie is a currency that exists as connections between users, (i.e. a [graph](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics))), unlike most every other currency, (e.g. the Dollar, Bitcoin, etc), in that these currencies exist as pools that users draw from where each unit is equivalent to the other. In the Whuffie currency graph, the nodes are users, companies and other entities, while edges are the uni-directional connections between nodes that users create.

There are two main actions supported by the API:
- **gifting** & **nixing** of Whuffie (aka expansion/retraction of uni-directional connections between users)
- **spending** Whuffie (sending Whuffie out of one account and into another along existing connections)

## Gifting
In a nutshell, gifting is making the choice to accept another user's Whuffie (in exchange of your own or another user's Whuffie you chose to accept) up to a user-defined amount.
- this creates a connection in the graph for that specified amount
- the more Whuffie you gift:
  - the more of another's Whuffie you choose to accept
  - the more Whuffie can be transferred through the currency graph through that connection (in both directions)
- this is analagous to "liking" a user's content, giving an IOU for a favor, thanking them with a reward, or generally "vouching" for another user's reputation
- the maximum amount of Whuffie that can be transferred along the connection in any one transaction is restricted to the amounts (known as *limits*) set by the two users on either side of the connection

The gifting (and nixing), in aggregate, creates a web of trust between users corresponding to whose Whuffie a user chooses to accept and in what amounts relative to other users.

## Spending
Sending whuffie to another user (i.e. making a payment to another user) requires that there be a connection (or path of connections) in the graph between you and the recipient. Whuffie will be spent along each connection in the path from you to the intended recipient, changing the balances of all contributing connections accordingly.

Put as simply as possible:
- spending whuffie "exhausts" a balance that exists on any connection considered a part of the transaction
  - e.g. gifting me for 5 whuffie means I can *directly* spend 5 Whuffie unto you *once*. At this point, my balance is 0 unto you and your balance is 5 Whuffie unto me
- since the connections create a graph, we can use graph traversal and maximum flow analysis to allow you to *indirectly* spend Whuffie unto users that do not direct connections with you
  - e.g. consider a graph where your parents gift you 100 Whuffie and you've gifted me 5 Whuffie. I can then spend 5 Whuffie unto your parents by:
    - first, spending 5 Whuffie unto you
    - then you (automatically, as part of the transaction) spend 5 Whuffie unto your parents.
  - After the transaction completes:
    - our edge's balance decreases from my perspective (and increases from your perspective)
    - the edge's balance between you and your parents goes up from their perspective (and decreases from your perspective).

## Implications
Following the rules of these simple actions as well as the transitive nature of spending along existing connections, we get a fully peer-to-peer social currency with some very interesting implications:
- you cannot gift yourself more Whuffie to spend into the rest of the network (since every user controls how much Whuffie they will accept and from whom it can be spent)
- you cannot use spam accounts to give yourself more Whuffie (since spam accounts themselves are unlikely to be gifted much from legitimate users)
- while there's no limitation to how much you can gift a user or how many users you can gift, **gifting lets the recipient spend Whuffie that you yourself were gifted by others**:
  - e.g. consider the aformentioned graph of me, you and your parents
    - you gift me an additional 1000 Whuffie
    - this lets me spend up to 95 more Whuffie unto your parents
  - spending 95 Whuffie unto your parents would leave you with:
    - 100 you can spend unto me
    - 900 I could spend unto you
    - but *0 you could spend unto your parents*
- as a result of this inherent disincentive to gifting, **Whuffie, in aggregate, will likely prove to be relatively inflation/deflation-proof**

Likewise, given the context in which Whuffie can be used (i.e. *anywhere* you can reward someone for some action), there are some other interesting social/political implications:
- "upvoting" or "liking" user-generated content with Whuffie disincentivizes voting-brigades and allows popularity of content to be a function of it's [PageRank](https://en.wikipedia.org/wiki/PageRank) (or another graph metric), rather than a simple, manipulable score
- "voting with your dollar" takes on an entirely new and empowered meaning:
  - a company's profitability and ability to spend in the greater network is a direct consequence of how much Whuffie they've been gifted by the rest of the network
  - reducing their profitability and amount of available credit is as simple as nixing the company (or nixing users connected to the company)
- while users may have been gifted in one context, they can be nixed in another, allowing for both positive and negative reputation to be calculated for a user for any given context
  - this also means the same Whuffie graph and currency can be used to map most if not all social interaction, on the web or IRL, allowing for global and local social/reputation metrics to be conducted, all in a peer-to-peer manner without a [central or privileged reputation authority](https://en.wikipedia.org/wiki/Social_Credit_System)
- because fungibility of your Whuffie has to be established by other users, **almost any kind of currency can be implemented on top of Whuffie**, allowing the currency to be limited to a particular community while still enjoying global interoperability with every other Whuffie sub-currency
- [others to be enumerated later]
