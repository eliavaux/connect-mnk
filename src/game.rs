#[allow(unused)]
use std::fmt::{self, Display, Formatter};

type Pos = (i32, i32);
type Dir = (i32, i32);

const DIRS: &[Dir] = &[
    (1, 0), (1, 1), (0, 1), (-1, 1),
    (-1, 0), (-1, -1), (0, -1), (1, -1)
];

fn next_pos((x, y): Pos, (dx, dy): Dir) -> Pos { (x + dx, y + dy) }

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Color {
    #[default]
    Red,
    Yellow
}

#[derive(Clone)]
pub struct Board<const WIDTH: usize, const HEIGHT: usize>(Vec<Option<Color>>);

impl<const WIDTH: usize, const HEIGHT: usize> Default for Board<WIDTH, HEIGHT> {
    fn default() -> Self { Self(vec![None; HEIGHT * WIDTH]) }
}

impl<const WIDTH: usize, const HEIGHT: usize> Display for Board<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut display= String::new();

        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                display += match self.0[HEIGHT * col + row] {
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

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    fn contains(pos: Pos) -> bool { pos.0 >= 0 && pos.1 >= 0 && pos.0 < WIDTH as i32 && pos.1 < HEIGHT as i32 }

    fn free_spaces(&self, column: usize) -> usize {
        self.0[HEIGHT * column..HEIGHT * (column + 1)]
            .iter()
            .filter(| &n| n.is_none())
            .count()
    }

    fn get_chip(&self, pos: Pos) -> Result<Option<Color>, ()> {
        if Board::<WIDTH, HEIGHT>::contains(pos) {
            Ok(self.0[HEIGHT * pos.0 as usize + pos.1 as usize])
        } else {
            Err(())
        }
    }

    fn insert(&mut self, column: usize, color: Color) -> Result<usize, &'static str> {
        if column >= WIDTH { return Err("Column does not exist") }
        let free_spaces = self.free_spaces(column);
        if free_spaces == 0 { return Err("Column is already full") }
        self.0[HEIGHT * column + free_spaces - 1] = Some(color);
        Ok(free_spaces - 1)
    }

    fn extract(&mut self, column: usize) -> Result<usize, &'static str> {
        if column >= WIDTH { return Err("Column does not exist") }
        let free_spaces = self.free_spaces(column);
        if free_spaces == HEIGHT { return Err("Column is empty") }
        self.0[HEIGHT * column + free_spaces] = None;
        Ok(free_spaces)
    }
}

#[derive(Clone, Debug)]
pub struct Score<const K: usize>([usize; K]);

#[derive(Clone, Default)]
pub struct Game<const M: usize, const N: usize, const K: usize> {
    pub board: Board<M, N>,
    pub turn: Color,
    pub move_list: Vec<usize>,
    pub score_list: Vec<(Score<K>, Score<K>)>,
}

impl<const M: usize, const N: usize, const K: usize> Game<M, N, K> {
    pub fn new() -> Self {
        Self {
            score_list: vec![(Score([0; K]), Score([0; K]))],
            ..Self::default()
        }
    }

    pub fn is_draw(&self) -> bool { self.move_list.len() >= M * N }

    pub fn not_turn(&self) -> Color { if self.turn == Color::Red { Color::Yellow } else { Color::Red } }

    pub fn last_move(&self) -> usize { *self.move_list.last().unwrap_or(&0) }

    pub fn last_score(&mut self) -> (Score<K>, Score<K>) { self.score_list.last().unwrap().clone() }

    pub fn minimax(&mut self, depth: usize) -> usize {
        if self.is_draw() { return 0 }
        if depth == 0 { return 0 }

        let score = 0;

        for column in 0..M {
            if let Ok(result) = self.run(column) {
                if let Some(_) = result {

                }
                println!("{}", self.board);
                self.minimax(depth - 1);
            }
            self.undo();
        }

        0
    }

    pub fn run(&mut self, column: usize) -> Result<Option<bool>, &'static str> {
        self.insert(column)?;
        self.turn = self.not_turn();

        if self.last_score().0.0[K-1] != 0 || self.last_score().1.0[K-1] != 0 { Ok(Some(true)) }
        else if self.is_draw() { Ok(Some(false)) }
        else { Ok(None) }
    }

    pub fn insert(&mut self, column: usize) -> Result<(), &'static str> {
        let row = self.board.insert(column, self.turn)?;
        self.move_list.push(column);
        let score = self.score((column as i32, row as i32));
        self.score_list.push(score);
        Ok(())
    }

    pub fn undo(&mut self) {
        if self.move_list.is_empty() { return }

        self.turn = self.not_turn();
        self.score_list.pop();
        let column = self.move_list.pop().unwrap();
        self.board.extract(column).unwrap();
    }

    fn score(&mut self, last_move: Pos) -> (Score<K>, Score<K>) {
        let mut colors= [None; 8];
        let mut lengths = [0; 8];
        let mut open = [false; 8];

        for (i, &dir) in DIRS.iter().enumerate() {
            let mut pos = next_pos(last_move, dir);
            if let Ok(Some(color)) = self.board.get_chip(pos) {
                colors[i] = Some(color);
                lengths[i] += 1;
                pos = next_pos(pos, dir);
                while self.board.get_chip(pos) == Ok(Some(color)) {
                    lengths[i] += 1;
                    pos = next_pos(pos, dir);
                }
            }
            if self.board.get_chip(pos) == Ok(None) { open[i] = true }
        }

        let turn = self.turn;
        let (mut score_red, mut score_yellow) = self.last_score();
        let (score_turn, score_other) = if turn == Color::Red {
            (&mut score_red, &mut score_yellow)
        } else {
            (&mut score_yellow, &mut score_red)
        };

        for i in 0..8 {
            let j = (i+4)%8;
            match colors[i] {
                Some(color) if color == turn => {
                    match colors[j] {
                        Some(color) if color == turn => {
                            if lengths[i] + lengths[j] >= K { score_turn.0[K-1] += 1 } // returns two instead of one, doesn't matter because game is over
                            else { score_turn.0[lengths[i] + lengths[j]] += open[i] as usize };
                        },
                        Some(_) => score_turn.0[lengths[i]] += open[i] as usize,
                        None => score_turn.0[lengths[i]] += open[i] as usize + open[j] as usize
                    }
                    score_turn.0[lengths[i]-1] -= 1 + open[i] as usize;
                },
                Some(_) => score_other.0[lengths[i]-1] -= 1,
                None => if colors[j] != Some(turn) { score_turn.0[0] += open[i] as usize }
            }
        }

        (score_red, score_yellow)
        // if open none and other side not turn color, +1T
        // count up
        //  if turn color and other side open none, +l+1T for each open side   *
        //  if turn color and other side turn color, +combined+1T for each open side
        // get rid of trash     *
        //  if turn color, -lT if closed, -lT -lT if open   *
        //  if other color, -lO     *
    }
}
