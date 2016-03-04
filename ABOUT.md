Here's how whuffie works:

Whuffie is a graph currency (as opposed to a "pooled" currency, like the Dollar, Bitcoin, etc) that exists as connections between users. Nodes are users, companies and other entities, while edges are the uni-directional connections between nodes.

There are two low-level actions supported:
- extension/retraction of whuffie (creating (or expanding)/destroying connections between users), known as gifting/nixing
- spending whuffie (exchanging whuffie between two users along existing connections)

In a nutshell, the extension/retraction action means that:
- whuffie is created when you gift another user some amount of whuffie, analagous to "vouching" for a user's reputation (the amount is known as a "limit")
- the more highly you rate someone's reputation, the more whuffie exists that can be transferred in the graph through that connection
- more whuffie can be created by gifting a user (increasing an existing connection's limit), and can likewise be destroyed by nixing a user (decreasing the connection's limit) or by destroying your uni-directional connection to the user altogether (you'll likely gift your parents more whuffie than me which could be more than what you gift users you do not know)
- the maximum amount of whuffie that can be spent along any connection is limited by the limits set by the two users 

The gifting/nixing actions creates a graph of trust (akin to a weighted web of trust) between users (or companies, etc).

Sending whuffie to another user (i.e. making a payment to another user) requires that there be a connection or path of connections in the graph between you and the recipient.
Put as simply as possible:
- spending whuffie "exhausts" the balance that exists on any connections considered a part of the transaction
- e.g. gifting me for 5 whuffie means I can spend 5 whuffie unto you *once*. At this point, my balance is 0 and yours is 5 whuffie unto me
- since the connections create a graph, we can use graph traversal and maximum flow (considering existing balances and set limits) to allow you to spend whuffie unto users that do not have balances directly with you
- e.g. consider a graph where your parents gift you 100 whuffie and you gift me 5 whuffie. I can then spend 5 whuffie unto your parents by first giving you 5 whuffie, then you (transparently) give 5 whuffie to your parents: our edge's balance decreases (from my perspective), the edge between you and your parents goes up (from their perspective).

Following these simple actions and limitations of creation and spending of whuffie, we get a fully peer-to-peer reputation currency with some very interesting benefits:
- spammers have little to no impact on the network since they are unlikely to be gifted much from users in the center of the graph
- you cannot give yourself more whuffie to spend into the rest of the network (since every user controls how much whuffie can be spent unto them and from whom it can be spent)
- while there's no limitation to how much you can gift a user, doing so lets them to spend whuffie that you yourself were gifted by others (e.g. consider the graph of me, you and your parents - gifting me an additional 1000 whuffie lets me spend 100 unto your parents, leaving you with 100 to spend to me, and 0 you can spend unto your parents)
- as a result of this inherent cost to gifting, in aggregate, whuffie is relatively inflation/deflation-proof (gifting indiscriminately allows others to spend whuffie you were gifted)

Likewise, there are some other interesting social/political benefits
- "voting with your dollar" takes on a new meaning since reducing a company's profitability and ability to spend in the network is as simple as nixing the company (or nixing users connected to the company)
- [others to be figured out later]
