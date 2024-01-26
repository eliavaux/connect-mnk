#[allow(unused)]

use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};

type Pos = (i32, i32);
type Dir = (i32, i32);

fn next_pos((x, y): Pos, (dx, dy): Dir) -> Pos { (x + dx, y + dy) }

const DIRS: &[Dir] = &[
    (1, 0), (1, 1),
    (0, 1), (1, -1),
];

#[derive(Default)]
pub struct Score {
    max_row: usize,
    num_max_rows: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Color {
    #[default]
    Red,
    Yellow
}

pub struct Board<const W: usize, const H: usize>(Vec<Option<Color>>);

impl<const W: usize, const H: usize> Default for Board<W, H> {
    fn default() -> Self { Self(vec![None; W * H]) }
}

impl<const W: usize, const H: usize> Display for Board<W, H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut display= String::new();

        for row in 0..H {
            for col in 0..W {
                display += match self.0[H * col + row] {
                    Some(Color::Red) => "X ",
                    Some(Color::Yellow) => "O ",
                    None => "_ ",
                };
            }
            display += "\n";
        }
        write!(f, "{display}")
    }
}

impl<const W: usize, const H: usize> Board<W, H> {
    fn contains(&self, pos: Pos) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < W as i32 && pos.1 < H as i32
    }
}

#[derive(Default)]
pub struct Game<const M: usize, const N: usize, const K: usize> {
    pub board: Board<M, N>,
    pub score: (Score, Score),
    pub last_move: Pos,
    pub turn: Color,
}

impl<const M: usize, const N: usize, const K: usize> Game<M, N, K> {
    pub fn new() -> Self { Game::default() }

    pub fn run(&mut self, column: usize) -> Result<Option<Color>, &'static str>{
        self.insert(column)?;

        if self.turn == Color::Red {
            self.score.0 = self.score();
            self.turn = Color::Yellow;
            if self.score.0.max_row >= K { return Ok(Some(Color::Red)) }
        } else {
            self.score.1 = self.score();
            self.turn = Color::Red;
            if self.score.1.max_row >= K { return Ok(Some(Color::Yellow)) }
        }

        Ok(None)
    }

    pub fn insert(&mut self, column: usize) -> Result<(), &'static str> {
       if column >= M { return Err("Column does not exist") }

        let free_spaces = self.board.0[N * column..N * (column + 1)]
            .iter()
            .filter(| &n| n.is_none())
            .count();

        if free_spaces == 0 { return Err("Column is already full") }

        self.board.0[N * column + free_spaces - 1] = Some(self.turn);
        self.last_move = (column as i32, free_spaces as i32 - 1);

        Ok(())
    }

    pub fn score(&self) -> Score {
        let max_row = DIRS.iter()
            .map(| &dir| self.count(self.last_move, dir))
            .max()
            .unwrap();

        let score = if self.turn == Color::Red { &self.score.0 } else { &self.score.1 };

        match max_row.cmp(&score.max_row) {
            Ordering::Greater => Score { max_row, num_max_rows: 1},
            Ordering::Equal => Score { max_row, num_max_rows: score.num_max_rows + 1},
            Ordering::Less => Score { ..*score },
        }
    }

    fn count(&self, pos: Pos, dir: Dir) -> usize {
        self.count_inner(pos, dir) + self.count_inner(pos, (-dir.0, -dir.1)) - 1
    }

    fn count_inner(&self, pos: Pos, dir: Dir) -> usize {
        if self.board.contains(pos) && self.board.0[N * pos.0 as usize + pos.1 as usize] == Some(self.turn) {
            1 + self.count_inner(next_pos(pos, dir), dir)
        } else { 0 }
    }
}