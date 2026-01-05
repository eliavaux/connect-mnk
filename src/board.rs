use std::{fmt::{self, Display, Formatter}, ops::{AddAssign, SubAssign}};

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq)]
pub enum Color {
    Red,
    Yellow,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Color::{Red, Yellow};
        write!(f, "{}", match self {
            Red => "Red",
            Yellow => "Yellow",
        })
    }
}

impl Color {
    fn other(&self) -> Self {
        use Color::{Red, Yellow};
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

impl AddAssign for Score {
    fn add_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self.0.iter_mut().zip(rhs.0) {
            *lhs += rhs
        }
    }
}

impl SubAssign for Score {
    fn sub_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self.0.iter_mut().zip(rhs.0) {
            *lhs -= rhs
        }
    }
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Debug)]
pub enum DeserializeError {
    EmptyInput,
    DifferentWidths,
    BadSymbol(char),
    UnreachablePosition
}

#[derive(Clone, Debug)]
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

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline(always)]
    pub fn k(&self) -> usize {
        self.k
    }

    #[inline(always)]
    pub fn turn(&self) -> Color {
        self.turn
    }

    pub fn minimax_iterative(&mut self, depth: usize) {
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

        todo!()
    }

    pub fn minimax_rec(&mut self, depth: usize) -> (Score, Vec<usize>) {
        let move_order = {
            let width = self.width as i32;
            let mut acc = width / 2;
            let mut sign = -1;
            let mut res = Vec::new();

            for i in 1..=width {
                res.push(acc as usize);
                acc += sign * i;
                sign = -sign;
            }

            res
        };
        let alpha = vec![i32::MIN; 4].into();
        let beta = vec![i32::MAX; 4].into();


        if self.turn() == Color::Red {
            self.minimax_rec_inner_red(depth, alpha, beta, &move_order)
        } else {
            self.minimax_rec_inner_yellow(depth, alpha, beta, &move_order)
        }
    }

    fn minimax_rec_inner_red(&mut self, depth: usize, mut alpha: Score, beta: Score, move_order: &[usize]) -> (Score, Vec<usize>) {
        if depth == 0 {
            return (self.last_score().clone(), Vec::new());
        }

        let mut best_moves = Vec::new();
        let mut best_move = 0;
        let mut best_score = vec![i32::MIN; self.k].into();

        for &i in move_order {
            let result = self.run_unchecked(i);
            match result {
                Some(GameState::InProgress) => {
                    let (new_score, moves) = self.minimax_rec_inner_yellow(depth - 1, alpha.clone(), beta.clone(), move_order);
                    self.undo_unchecked();

                    if new_score > best_score {
                        best_score = new_score.clone();
                        best_moves = moves;
                        best_move = i;
                        alpha = alpha.max(new_score);
                        if beta <= alpha { break }
                    }
                    // dbg!(&new_score, &best_score);
                },
                Some(GameState::Win(_)) => {
                    self.undo_unchecked();
                    let best_score = vec![i32::MAX - 1; self.k].into();
                    return (best_score, vec![i])
                },
                Some(GameState::Draw) => {
                    self.undo_unchecked();
                    return (vec![0; self.k].into(), vec![i])
                }
                None => continue,
            }
        }

        best_moves.push(best_move);

        // dbg!(&best_moves, &current_score);
        (best_score, best_moves)
    }

    fn minimax_rec_inner_yellow(&mut self, depth: usize, alpha: Score, mut beta: Score, move_order: &[usize]) -> (Score, Vec<usize>) {
        if depth == 0 {
            return (self.last_score().clone(), Vec::new());
        }

        let mut best_moves = Vec::new();
        let mut best_move = 0;
        let mut best_score = vec![i32::MAX; self.k].into();

        for &i in move_order {
            let result = self.run_unchecked(i);
            match result {
                Some(GameState::InProgress) => {
                    let (new_score, moves) = self.minimax_rec_inner_red(depth - 1, alpha.clone(), beta.clone(), move_order);
                    self.undo_unchecked();
                    // dbg!(&new_score, &best_score);
                    if new_score < best_score {
                        best_score = new_score.clone();
                        best_moves = moves;
                        best_move = i;
                        beta = beta.min(new_score);
                        if beta <= alpha { break }
                    }
                },
                Some(GameState::Win(_)) => {
                    self.undo_unchecked();
                    let best_score = vec![i32::MIN + 1; self.k].into();
                    return (best_score, vec![i])
                },
                Some(GameState::Draw) => {
                    self.undo_unchecked();
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

        let score = self.score((column, full_spaces));
        self.score_list.push(score);
        self.board[self.height * column + full_spaces] = Some(color);
        self.full_spaces[column] += 1;
        self.move_list.push(column);
        
        if *self.last_score().0.last().unwrap() != 0 {
            self.game_state = GameState::Win(self.turn());
        } else if self.move_list.len() == self.height() * self.width() {
             self.game_state = GameState::Draw
        }

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

    pub fn score(&mut self, last_move: (usize, usize)) -> Score {
        let ks1 = self.k - 1;
        let (x, y) = last_move;

        let south = ks1.min(y);
        let north = ks1.min(self.height() - y - 1);
        let west = ks1.min(x);
        let east = ks1.min(self.width() - x - 1);

        let sw = south.min(west);
        let ne = north.min(east);
        let nw = north.min(west);
        let se = south.min(east);
        

        let vertical: Vec<&Option<Color>> = self.board.iter()
            .skip(self.height() * x + y - south)
            .take(south + 1 + north)
            .collect();
        let horizontal: Vec<&Option<Color>> = self.board.iter()
            .skip(self.height() * (x - west) + y)
            .step_by(self.height()).take(west + 1 + east)
            .collect();
        let sw_ne: Vec<&Option<Color>> = self.board.iter()
            .skip(self.height() * (x - sw) + y - sw)
            .step_by(self.height() + 1).take(sw + 1 + ne)
            .collect();
        let nw_se: Vec<&Option<Color>> = self.board.iter()
            .skip(self.height() * (x - nw) + y + nw)
            .step_by(self.height() - 1).take(nw + 1 + se)
            .collect();

        /* dbg!(north);
        dbg!(south);
        dbg!(west);
        dbg!(east);

        dbg!(&vertical);
        dbg!(&horizontal);
        dbg!(&sw_ne);
        dbg!(&nw_se); */


        let score_vertical = self.score_line(&vertical);
        let score_horizontal = self.score_line(&horizontal);
        let score_sw_ne = self.score_line(&sw_ne);
        let score_nw_se = self.score_line(&nw_se);

        let mut score = self.last_score().clone();

        if self.turn() == Color::Red {
            score += score_vertical;
            score += score_horizontal;
            score += score_sw_ne;
            score += score_nw_se;
        } else {
            score -= score_vertical;
            score -= score_horizontal;
            score -= score_sw_ne;
            score -= score_nw_se;
        }

        score
    }

    fn score_line(&self, line: &[&Option<Color>]) -> Score {
        let mut score: Score = vec![0; self.k()].into();

        if line.len() < self.k() {
            return score;
        }

        let mut count_turn = 0;
        let mut count_other = 0;


        for color in line.iter().take(self.k()).cloned().flatten() {
            if *color == self.turn() {
                count_turn += 1;
            } else {
                count_other += 1;
            }
        }

        if count_other == 0 {
            if count_turn != 0 {
                score.0[count_turn - 1] -= 1;
            }
            score.0[count_turn] += 1;
        } else if count_turn == 0 {
            score.0[count_other - 1] += 1;
        }

        let mut tail = 0;
        for head in self.k()..line.len() {
            let cell_head = line[head];
            let cell_tail = line[tail];
            // dbg!(cell_head);
            // dbg!(head);

            if let Some(color) = cell_head {
                if *color == self.turn() {
                    count_turn += 1;
                } else {
                    count_other += 1;
                }
            }

            if let Some(color) = cell_tail {
                if *color == self.turn() {
                    count_turn -= 1;
                } else {
                    count_other -= 1;
                }
            }

            if count_other == 0 {
                if count_turn != 0 {
                    score.0[count_turn - 1] -= 1;
                }
                score.0[count_turn] += 1;
            } else if count_turn == 0 {
                score.0[count_other - 1] += 1;
            }

            tail += 1;
        }

        return score
    }

    pub fn serialize(&self) -> String {
        format!("{self}")
    }

    pub fn deserialize(input: &str, k: usize) -> Result<Self, DeserializeError> {
        use DeserializeError::{EmptyInput, DifferentWidths, BadSymbol, UnreachablePosition};

        let mut board = Vec::new();
        let mut width = None;

        // Deserialize the board
        for line in input.lines().rev() {
            let line = line.trim();
            if line.is_empty() { continue }
            let mut row = Vec::new();
            for symbol in line.chars() {
                use Color::{Red, Yellow};

                let field = match symbol {
                    'X' => Some(Red),
                    'O' => Some(Yellow),
                    '_' => None,
                    ' ' => continue,
                    s => return Err(BadSymbol(s)),
                };
                row.push(field);
            }

            if let Some(width) = width {
                if row.len() != width {
                    dbg!(width, row.len());
                    return Err(DifferentWidths)
                }
            } else {
                width = Some(row.len())
            }

            board.push(row);
        }

        let Some(width) = width else { return Err(EmptyInput) };

        let height = board.len();


        // Reconstruct the move list
        let mut move_list = Vec::new();
        let mut indexes = vec![0; width].into_boxed_slice();
        let mut turn = Color::Red;

        loop {
            let mut found = false;
            for i in 0..width {
                if indexes[i] == height { continue }
                if board[indexes[i]][i] == Some(turn) {
                    move_list.push(i);
                    indexes[i] += 1;
                    found = true;
                    turn = turn.other();
                }
            }
            if !found { break }
        }

        for i in 0..width {
            if indexes[i] == height { continue }
            if board[indexes[i]][i] != None {
                return Err(UnreachablePosition)
            }
        }

        let mut board = Self::new(width, height, k);

        for i in move_list {
            board.run_unchecked(i);
        }

        Ok(board)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Color::{Red, Yellow};
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
