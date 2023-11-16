use std::{error, fmt, io};

pub const COLS: usize = 7;
const ROWS: usize = 6;
const CONNECT: usize = 4;

#[derive(Clone, PartialEq, Debug)]
enum Color {
    Blue,
    Red,
}

pub struct Game {
    board: [[Option<Color>; ROWS]; COLS],
    player_turn: Color,
    last_turn: (usize, usize),
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = String::new();

        for column in 0..ROWS {
            display += "\n";
            for row in 0..COLS {
                if self.board[row][column].is_none() {
                    display += "_ ";
                } else if self.board[row][column] == Some(Color::Blue) {
                    display += "X ";
                } else {
                    display += "O ";
                }
            }
        }
        write!(f, "{display}")
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Default::default(),
            player_turn: Color::Blue,
            last_turn: (0, 0),
        }
    }

    pub fn run(&mut self) -> Result<bool, Box<dyn error::Error>> {
        println!("{:?}'s turn", self.player_turn);

        let input = input()?.trim().to_lowercase();

        if input == *"quit" {
            return Ok(true);
        }

        let num: usize = input.parse()?;
        self.next_turn(num)?;
        println!("{self}");

        if self.did_win() {
            println!("{:?} won the game!", self.player_turn);
            return Ok(true);
        }

        if self.player_turn == Color::Blue {
            self.player_turn = Color::Red
        } else {
            self.player_turn = Color::Blue
        }

        Ok(false)
    }

    fn next_turn(&mut self, column: usize) -> Result<(), String> {
        if !(1..=COLS).contains(&column) {
            return Err(format!("Please select a column between 1 and {COLS}"));
        }

        let free_spaces_in_row = self.board[column - 1]
            .iter()
            .filter(|&n| n.is_none())
            .count();

        if free_spaces_in_row == 0 {
            return Err(String::from("Column is already full"));
        }

        self.last_turn = (column - 1, free_spaces_in_row - 1);
        self.board[self.last_turn.0][self.last_turn.1] = Some(self.player_turn.clone());

        Ok(())
    }

    fn did_win(&self) -> bool {
        let board = &self.board;
        let (column, row) = self.last_turn;

        // lateral
        let mut chips_lat = 1;
        let mut diff = 1;

        while column as i32 - diff as i32 >= 0
            && diff < CONNECT
            && board[column - diff][row] == Some(self.player_turn.clone())
        {
            chips_lat += 1;
            diff += 1;
        }

        diff = 1;
        while column + diff < COLS
            && diff < CONNECT
            && board[column + diff][row] == Some(self.player_turn.clone())
        {
            chips_lat += 1;
            diff += 1;
        }
        if chips_lat >= CONNECT {
            return true;
        }

        // vertical
        let mut chips_vert = 1;
        let mut diff = 1;

        while row + diff < ROWS
            && diff < CONNECT
            && board[column][row + diff] == Some(self.player_turn.clone())
        {
            chips_vert += 1;
            diff += 1;
        }
        if chips_vert >= CONNECT {
            return true;
        }

        // diagonal incline
        let mut chips_incline = 1;
        let mut diff = 1;

        while column as i32 - diff as i32 >= 0
            && row + diff < ROWS
            && board[column - diff][row + diff] == Some(self.player_turn.clone())
            && diff < CONNECT
        {
            chips_incline += 1;
            diff += 1;
        }

        diff = 1;
        while column + diff < COLS
            && row as i32 - diff as i32 >= 0
            && board[column + diff][row - diff] == Some(self.player_turn.clone())
            && diff < CONNECT
        {
            chips_incline += 1;
            diff += 1;
        }
        if chips_incline >= CONNECT {
            return true;
        }

        // diagonal decline
        let mut chips_decline = 1;
        let mut diff = 1;

        while column + diff < COLS
            && row + diff < ROWS
            && board[column + diff][row + diff] == Some(self.player_turn.clone())
            && diff < CONNECT
        {
            chips_decline += 1;
            diff += 1;
        }

        diff = 1;
        while column as i32 - diff as i32 >= 0
            && row as i32 - diff as i32 >= 0
            && board[column - diff][row - diff] == Some(self.player_turn.clone())
            && diff < CONNECT
        {
            chips_decline += 1;
            diff += 1;
        }
        if chips_decline >= CONNECT {
            return true;
        }

        false
    }
}

fn input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input)
}
