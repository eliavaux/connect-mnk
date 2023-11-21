use std::{error, fmt, io};

pub const COLS: usize = 7;
const ROWS: usize = 6;
const CONNECT: usize = 4;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Color {
    Blue,
    Red,
}

pub struct Game {
    board: [Option<Color>; ROWS * COLS],
    player_turn: Color,
    last_turn: (usize, usize),
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = String::new();

        for i in 0..self.board.len() {
            // transposing the board from column-major to row-major
            if self.board[(i % COLS) * ROWS + i / COLS].is_none() { display += "_ "}
            else if self.board[(i % COLS) * ROWS + i / COLS] == Some(Color::Blue) { display += "X " }
            else if self.board[(i % COLS) * ROWS + i / COLS] == Some(Color::Red) { display += "O " }

            if i % COLS == COLS - 1 { display += "\n"; }
        }

        write!(f, "{display}")
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: [None; COLS * ROWS],
            player_turn: Color::Blue,
            last_turn: (0, 0),
        }
    }
}

impl Game {

    pub fn run(&mut self) -> Result<bool, Box<dyn error::Error>> {
        println!("{:?}'s turn", self.player_turn);

        let input = input()?.trim().to_lowercase();

        if input == *"quit" {
            return Ok(false);
        }

        let num: usize = input.parse()?;
        self.insert(num)?;
        println!("{self}");

        if self.evaluate() == 1000 {
            println!("{:?} won the game!", self.player_turn);
            return Ok(false);
        }

        if self.player_turn == Color::Blue { self.player_turn = Color::Red }
        else { self.player_turn = Color::Blue }

        Ok(true)
    }

    fn insert(&mut self, column: usize) -> Result<(), String> {
        if !(1..=COLS).contains(&column) {
            return Err(format!("Please select a column between 1 and {COLS}"));
        }

        let free_spaces = self.board[ROWS * (column - 1)..ROWS * column]
            .iter()
            .filter(|&n| n.is_none())
            .count();

        if free_spaces == 0 {
            return Err(String::from("Column is already full"));
        }

        self.last_turn = (column - 1, free_spaces - 1);
        self.board[ROWS * self.last_turn.0 + self.last_turn.1] = Some(self.player_turn);

        Ok(())
    }

    fn evaluate(&self) -> usize {
        let board = &self.board;
        let (column, row) = self.last_turn;
        let turn_color = Some(self.player_turn);

        // lateral
        let mut chips_lat = 1;
        let mut diff = 1;

        while 1 + column - diff > 0
            && diff < CONNECT
            && board[ROWS * (column - diff) + row] == turn_color
        {
            chips_lat += 1;
            diff += 1;
        }

        diff = 1;
        while column + diff < COLS
            && diff < CONNECT
            && board[ROWS * (column + diff) + row] == turn_color
        {
            chips_lat += 1;
            diff += 1;
        }

        if chips_lat >= CONNECT { return 1000; }


        // vertical
        let mut chips_vert = 1;
        let mut diff = 1;

        while row + diff < ROWS
            && diff < CONNECT
            && board[ROWS * column + row + diff] == turn_color
        {
            chips_vert += 1;
            diff += 1;
        }

        if chips_vert >= CONNECT { return 1000; }


        // diagonal incline
        let mut chips_incline = 1;
        let mut diff = 1;

        while 1 + column - diff > 0
            && row + diff < ROWS
            && board[ROWS * (column - diff) + row + diff] == turn_color
            && diff < CONNECT
        {
            chips_incline += 1;
            diff += 1;
        }

        diff = 1;
        while column + diff < COLS
            && 1 + row - diff > 0
            && board[ROWS * (column + diff) + row - diff] == turn_color
            && diff < CONNECT
        {
            chips_incline += 1;
            diff += 1;
        }

        if chips_incline >= CONNECT { return 1000; }


        // diagonal decline
        let mut chips_decline = 1;
        let mut diff = 1;

        while column + diff < COLS
            && row + diff < ROWS
            && board[ROWS * (column + diff) + row + diff] == turn_color
            && diff < CONNECT
        {
            chips_decline += 1;
            diff += 1;
        }

        diff = 1;
        while 1 + column - diff > 0
            && 1 + row - diff > 0
            && board[ROWS * (column - diff) + row - diff] == turn_color
            && diff < CONNECT
        {
            chips_decline += 1;
            diff += 1;
        }

        if chips_decline >= CONNECT { return 1000; }

        chips_lat.max(chips_vert).max(chips_incline).max(chips_decline)
    }
}

fn input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input)
}
