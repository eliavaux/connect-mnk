## Description

A simple [m,n,k-game](https://en.wikipedia.org/wiki/M,n,k-game) with Connect Four rules (i.e. every chip must be placed at the lowest position).

The size of the board (M * N) and the number of chips in a row required to win (K) are defined when initializing the board struct. 
A working example game is set up in `src/main.rs`.

The undo function undoes the last move. The board is one vector with column-major order. A score list keeps track of the number of open chains on the board. When a chain's sides are blocked off, it's no longer open. The score vector is a list, where each element represents the number of chains with the length of its index. If a player creates a chain of k-in-a-row, they win.

## Thoughts

I plan to implement a minimax algorithm for an arbitrary board size in the future.
