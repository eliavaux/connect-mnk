## Description

A simple [m,n,k-game](https://en.wikipedia.org/wiki/M,n,k-game) with Connect Four rules (i.e. every chip must be placed at the lowest position).

The size of the board (M * N) and the number of chips in a row required to win (K) are defined when initializing the board struct.
It comes with a working alpha-beta pruning minimax algorithm. A working example game is set up in `src/main.rs`.

### Features

With the `play()` function, you can set up a game between two players, a player versus the computer or two computers playing against each other. 
The undo function undoes the last move.

The board is one vector with column-major order.
A score list keeps track of the number of open chains on the board.
When a chain's sides are blocked off, it's no longer open.
The score vector is a list, where each element represents the number of chains with the length of its index.
If a player creates a chain of k-in-a-row, they win.


## Thoughts

The score mechanic is probably over-engineered and I should have just went with the highest row of each player, since you don't really need more for a minimax algorithm.
A connect-k game is rather simple and you can often just search the entire tree directly.

That said, the minimax algorithm works well and is very fun to play against, even though it beats me every time.
