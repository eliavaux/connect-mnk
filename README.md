## Description

A simple [m,n,k-game](https://en.wikipedia.org/wiki/M,n,k-game) with Connect Four rules (i.e. every token must be placed at the lowest position).

The size of the board (M * N) and the number of chips of the same color in a row required to win (K) are defined when initializing the board struct. An example for how a game could be set up is given in `src/main.rs` 

To check if a player reached k-in-a-row, the game checks the board for K chips horizontally, vertically and diagonally from the position of the last move. The score tracks the longest chain of chips of both players and how many longest chains they have. The board is a Vector which represents the 2D-board with column-major order. 

## Thoughts

I plan to implement a minimax algorithm for an arbitrary board size in the future.
