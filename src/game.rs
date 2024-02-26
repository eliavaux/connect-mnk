#[allow(unused)]

use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};

type Pos = (usize, usize);
type IPos = (i32, i32);
type Dir = (i32, i32);

const DIRS: &[Dir] = &[
    (1, 0), (1, 1), (0, 1), (-1, 1),
    (-1, 0), (-1, -1), (0, -1), (1, -1)
];

fn next_pos((x, y): IPos, (dx, dy): Dir) -> IPos { (x + dx, y + dy) }
fn to_ipos(pos: &Pos) -> IPos { (pos.0 as i32, pos.1 as i32) }

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Color {
    #[default]
    Red,
    Yellow
}

pub struct Board<const HEIGHT: usize, const WIDTH: usize>(Vec<Option<Color>>);

impl<const HEIGHT: usize, const WIDTH: usize> Default for Board<HEIGHT, WIDTH> {
    fn default() -> Self { Self(vec![None; HEIGHT * WIDTH]) }
}

impl<const HEIGHT: usize, const WIDTH: usize> Display for Board<HEIGHT, WIDTH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut display= String::new();

        for row in 0..WIDTH {
            for col in 0..HEIGHT {
                display += match self.0[WIDTH * col + row] {
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

impl<const HEIGHT: usize, const WIDTH: usize> Board<HEIGHT, WIDTH> {
    fn contains(pos: IPos) -> bool { pos.0 >= 0 && pos.1 >= 0 && pos.0 < HEIGHT as i32 && pos.1 < WIDTH as i32 }

    fn free_spaces(&self, column: usize) -> usize {
        self.0[WIDTH * column..WIDTH * (column + 1)]
            .iter()
            .filter(| &n| n.is_none())
            .count()
    }
}

#[derive(Debug)]
pub struct Score<const K: usize>([usize; K]);
impl<const K: usize> Default for Score<K> { fn default() -> Self { Self([0; K]) } }


#[derive(Default)]
pub struct Game<const M: usize, const N: usize, const K: usize> {
    pub board: Board<M, N>,
    pub score: (Score<K>, Score<K>),
    pub turn: Color,
    pub move_list: Vec<Pos>,
}

impl<const M: usize, const N: usize, const K: usize> Game<M, N, K> {
    pub fn new() -> Self { Self::default() }

    pub fn last_move(&self) -> &Pos { self.move_list.last().unwrap_or(&(0, 0)) }

    // game doesn't end when the board is full
    pub fn run(&mut self, column: usize) -> Result<Option<bool>, &'static str> {
        self.insert(column)?;

        self.turn = if self.turn == Color::Red { Color::Yellow } else { Color::Red };

        if self.score.0.0[K-1] != 0 || self.score.1.0[K-1] != 0 { Ok(Some(true)) }
        else if self.move_list.len() == M * N { Ok(Some(false)) } else { Ok(None)}
    }

    pub fn undo(&mut self) {
        if self.move_list.len() == 0 { return () }

        let &(col, row) = self.last_move();

        self.turn = if self.turn == Color::Red { Color::Yellow } else { Color::Red };

        self.score(true);
        self.move_list.pop();
        self.board.0[N * col + row] = None;
    }

    pub fn insert(&mut self, column: usize) -> Result<(), &'static str> {
        if column >= M { return Err("Column does not exist") }

        let free_spaces = self.board.free_spaces(column);
        if free_spaces == 0 { return Err("Column is already full") }

        self.board.0[N * column + free_spaces - 1] = Some(self.turn);
        self.move_list.push((column, free_spaces - 1));

        self.score(false);
        Ok(())
    }

    fn score(&mut self, undo: bool) {
        let last_move = to_ipos(self.last_move());

        let result = self.count(last_move);
        let combined: Vec<usize> = (0..4).map(|x| (result[x] + result[x+4]).min(K-1)).collect();
        let score = if self.turn == Color::Red { &mut self.score.0.0 } else { &mut self.score.1.0 };

        if undo {
            for k in combined { score[k] -= 1 }
            for k in result { if k != 0 { score[k-1] += 1 } }
        } else {
            for k in combined { score[k] += 1 }
            for k in result { if k != 0 { score[k-1] -= 1 } }
        }
    }

    fn count(&self, pos: IPos) -> Vec<usize> { DIRS.iter().map(|&dir| self.count_inner(pos, dir) - 1).collect() }

    fn count_inner(&self, pos: IPos, dir: Dir) -> usize {
        if Board::<M, N>::contains(pos) && self.board.0[N * pos.0 as usize + pos.1 as usize] == Some(self.turn) {
            1 + self.count_inner(next_pos(pos, dir), dir)
        } else { 0 }
    }
}
