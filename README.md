## Description

A simple [m,n,k-game](https://en.wikipedia.org/wiki/M,n,k-game) with Connect Four rules (i.e. every token must be placed in the lowest possible position).

The size of the board (m * n) as well as the number of stones of the same color in a row required to win (k) can be changed
inside the `src/game.rs` file.

To check who won the game, each move, the code looks for stones of the same color left, right, down and diagonal to the
stone placed that turn. If the end of the board is reached, the color of the stone is different or k-in-a-row has already occured,
the search can stop early.

## Thoughts

While still a fairly basic game I programmed for fun, I do plan on adding a minmax algorithm to play against. Perhaps I will give
the project a GUI, I could also expand the m-by-n board to higher dimensions.
