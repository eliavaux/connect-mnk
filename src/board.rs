use std::fmt::{self, Display, Formatter};
use Color::*;

type Pos = (usize, usize);

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq)]
pub enum Color {
    Red,
    Yellow,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Red => "Red",
            Yellow => "Yellow",
        })
    }
}

impl Color {
    fn other(&self) -> Self {
        match self {
            Red => Yellow,
            Yellow => Red
        }
    }    
}

type Field = Option<Color>;


#[derive(PartialEq, Eq)]
#[derive(Debug, Clone)]
pub struct Score(Box<[i32]>);

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.iter().rev().cmp(other.0.iter().rev())
    }
}

impl From<Vec<i32>> for Score {
    fn from(value: Vec<i32>) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Copy)]
pub enum GameState {
    Win(Color),
    Draw,
    InProgress,
}

pub enum InsertError {
    InvalidColumn,
    ColumnFull,
}

pub enum ExtractError {
    InvalidColumn,
    ColumnEmpty,
}

#[derive(Clone)]
pub struct Game {
    width: usize,
    height: usize,
    k: usize,
    turn: Color,
    board: Box<[Field]>,
    full_spaces: Box<[usize]>,
    move_list: Vec<usize>,
    score_list: Vec<Score>,
    game_state: GameState
}

impl Game {
    pub fn new(m: usize, n: usize, k: usize) -> Self {
        assert!(m >= k && n >= k);
        Self {
            width: m,
            height: n,
            k: k,
            board: vec![None; m * n].into(),
            full_spaces: vec![0; m].into(),
            score_list: vec![vec![0;k].into()],
            turn: Color::Red,
            move_list: Vec::new(),
            game_state: GameState::InProgress,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn k(&self) -> usize {
        self.k
    }

    #[inline(always)]
    pub fn turn(&self) -> Color {
        self.turn
    }

    pub fn minimax(&mut self, depth: usize) {
        let mut dfs_stack: Box<[_]> = vec![0; depth].into();

        let mut i = 0;
        for _ in 0..depth {
            while self.run_unchecked(i).is_none() && i < self.width {
                self.run_unchecked(i);
                i += 1;
            }
        }

        loop {
            dfs_stack[depth - 1] += 1;
            self.undo();

            let mut depth_index = depth - 1;
            while dfs_stack[depth_index] == self.width {
                let Some(_) = self.undo() else { return };

                dfs_stack[depth_index] = 0;
                depth_index -= 1;
                dfs_stack[depth_index] += 1;
            }
            for i in depth_index..depth {
                self.run_unchecked(dfs_stack[i]);
            }

            if self.board.iter().filter(|x| x.is_some()).count() == 7 {
                println!("{dfs_stack:?}");
                println!("{self}");
                println!("{:?}", self.last_score());
                println!();
            }
        }
    }

    pub fn minimax_rec(&mut self, depth: usize, move_order: &[usize]) -> (Score, Vec<usize>) {
        let alpha = vec![i32::MIN; 4].into();
        let beta = vec![i32::MAX; 4].into();
        self.minimax_rec_inner(depth, alpha, beta, move_order)
    }

    fn minimax_rec_inner(&mut self, depth: usize, mut alpha: Score, mut beta: Score, move_order: &[usize]) -> (Score, Vec<usize>) {
        let mut best_moves = Vec::new();
        let mut best_move = 0;

        if depth == 0 {
            return (self.last_score().clone(), best_moves);
        }

        let mut best_score = if self.turn() == Color::Red {
            vec![i32::MIN; self.k].into()
        } else {
            vec![i32::MAX; self.k].into()
        };
        let current_turn = self.turn();

        for &i in move_order {
            let result = self.run_unchecked(i);
            // println!("{self}");
            // println!("{:?}", self.last_score());
            match result {
                Some(GameState::InProgress) => {
                    let (new_score, moves) = self.minimax_rec_inner(depth - 1, alpha.clone(), beta.clone(), move_order);
                    self.undo();
                    // dbg!(&new_score, &best_score);
                    if self.turn() == Color::Red {
                        if new_score > best_score {
                            best_score = new_score.clone();
                            best_moves = moves;
                            best_move = i;
                            alpha = alpha.max(new_score);
                            if beta <= alpha { break }
                        }
                    } else {
                        if new_score < best_score {
                            best_score = new_score.clone();
                            best_moves = moves;
                            best_move = i;
                            beta = beta.min(new_score);
                            if beta <= alpha { break }
                        }
                    }
                },
                Some(GameState::Win(col)) => {
                    self.undo();
                    let best_score = if self.turn() == Color::Red {
                        vec![i32::MAX - 1; self.k].into()
                    } else {
                        vec![i32::MIN + 1; self.k].into()
                    };
                    return (best_score, vec![i])
                },
                Some(GameState::Draw) => {
                    self.undo();
                    return (vec![0; self.k].into(), vec![i])
                }
                None => continue,
            }
        }

        best_moves.push(best_move);

        // dbg!(&best_moves, &current_score);
        (best_score, best_moves)
    }

    pub fn run(&mut self, column: usize) -> Result<GameState, InsertError> {
        self.insert(column, self.turn())?;
        self.turn = self.turn.other();

        Ok(self.game_state)
    }

    fn run_unchecked(&mut self, column: usize) -> Option<GameState> {
        self.insert_unchecked(column, self.turn())?;
        self.turn = self.turn.other();

        Some(self.game_state)
    }
    
    pub fn undo(&mut self) -> Option<()> {
        let last_move = self.move_list.pop()?;
        self.turn = self.turn().other();
        self.extract_unchecked(last_move);
        self.score_list.pop();
        self.game_state = GameState::InProgress; // This is techincally wrong, but I need it for minimax to work

        Some(())
    }
    
    fn undo_unchecked(&mut self) {
        let last_move = self.move_list.pop().unwrap();
        self.turn = self.turn.other();
        self.extract_unchecked(last_move);
        self.score_list.pop();
        self.game_state = GameState::InProgress; // This is techincally wrong, but I need it for minimax to work
    }

    fn insert(&mut self, column: usize, color: Color) -> Result<(usize, usize), InsertError> {
        if column >= self.width { return Err(InsertError::InvalidColumn) }

        let Some(row) = self.insert_unchecked(column, color) else {
            return Err(InsertError::ColumnFull)
        };

        Ok((column, row))
    }

    fn insert_unchecked(&mut self, column: usize, color: Color) -> Option<usize> {
        let full_spaces = self.full_spaces[column];
        if full_spaces == self.height { return None }

        let score = self.score((column, full_spaces), color);
        if *score.0.last().unwrap() != 0 {
            self.game_state = GameState::Win(self.turn());
        }
        self.score_list.push(score);
        self.board[self.height * column + full_spaces] = Some(color);
        self.full_spaces[column] += 1;
        self.move_list.push(column);

        Some(full_spaces)
    }

    fn extract(&mut self, column: usize) -> Result<(), ExtractError> {
        if column >= self.width { return Err(ExtractError::InvalidColumn) }

        let full_spaces = &mut self.full_spaces[column];
        if full_spaces == &0 { return Err(ExtractError::ColumnEmpty) }

        *full_spaces -= 1;
        self.board[self.height * column + *full_spaces] = None;
                
        Ok(())
    }

    fn extract_unchecked(&mut self, column: usize) {
        let full_spaces = &mut self.full_spaces[column];

        *full_spaces -= 1;
        self.board[self.height * column + *full_spaces] = None;
    }
    
    pub fn last_score(&self) -> &Score {
        self.score_list.last().unwrap()
    }

    pub fn score(&mut self, last_move: Pos, color: Color) -> Score {
        let width = self.width;
        let height = self.height;
        let k = self.k - 1;
        let turn = self.turn();
        let (x, y) = last_move;

        let mut score = self.last_score().clone();

        let south = k.min(y);
        let north = k.min(height - y - 1);
        let west = k.min(x);
        let east = k.min(width - x - 1);

        let sw = south.min(west);
        let ne = north.min(east);
        let nw = north.min(west);
        let se = south.min(east);
        

        let mut vertical: Vec<&Option<Color>> = self.board.iter()
            .skip(height * x + y - south)
            .take(south + 1 + north)
            .collect();
        let mut horizontal: Vec<&Option<Color>> = self.board.iter()
            .skip(height * (x - west) + y)
            .step_by(height).take(west + 1 + east)
            .collect();
        let mut sw_ne: Vec<&Option<Color>> = self.board.iter()
            .skip(height * (x - sw) + y - sw)
            .step_by(height + 1).take(sw + 1 + ne)
            .collect();
        let mut nw_se: Vec<&Option<Color>> = self.board.iter()
            .skip(height * (x - nw) + y + nw)
            .step_by(height - 1).take(nw + 1 + se)
            .collect();

        /* dbg!(north);
        dbg!(south);
        dbg!(west);
        dbg!(east);

        dbg!(&vertical);
        dbg!(&horizontal);
        dbg!(&sw_ne);
        dbg!(&nw_se); */


        self.score_line(&vertical, &mut score);
        self.score_line(&horizontal, &mut score);
        self.score_line(&sw_ne, &mut score);
        self.score_line(&nw_se, &mut score);
        
        score
    }

    fn score_line(&self, line: &[&Option<Color>], score: &mut Score) {
        let k = self.k;
        let turn = self.turn();

        let mut score_diff: Score = vec![0; self.width].into();
        let mut count_turn = 0;
        let mut count_other = 0;

        if line.len() < k {
            return;
        }

        for color in line.iter().take(k).cloned().flatten() {
            if *color == turn {
                count_turn += 1;
            } else {
                count_other += 1;
            }
        }

        if count_other == 0 {
            if count_turn != 0 {
                score_diff.0[count_turn - 1] -= 1;
            }
            score_diff.0[count_turn] += 1;
        } else if count_turn == 0 {
            score_diff.0[count_other - 1] += 1;
        }

        let mut tail = 0;
        for head in k..line.len() {
            let cell_head = line[head];
            let cell_tail = line[tail];
            // dbg!(cell_head);
            // dbg!(head);

            if let Some(color) = cell_head {
                if *color == turn {
                    count_turn += 1;
                } else {
                    count_other += 1;
                }
            }

            if let Some(color) = cell_tail {
                if *color == turn {
                    count_turn -= 1;
                } else {
                    count_other -= 1;
                }
            }

            if count_other == 0 {
                if count_turn != 0 {
                    score_diff.0[count_turn - 1] -= 1;
                }
                score_diff.0[count_turn] += 1;
            } else if count_turn == 0 {
                score_diff.0[count_other - 1] += 1;
            }

            tail += 1;
        }

        if turn == Color::Red {
            for (a, b) in score.0.iter_mut().zip(score_diff.0) {
                *a += b;
            }
        } else {
            for (a, b) in score.0.iter_mut().zip(score_diff.0) {
                *a -= b;
            }
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                match self.board[self.height * (col + 1) - row - 1] {
                    Some(Red) => write!(f, "X")?,
                    Some(Yellow) => write!(f, "O")?,
                    None => write!(f, "_")?,
                }
                write!(f, " ")?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}
