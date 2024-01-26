use std::io::stdin;
use crate::game::Board;

pub mod game;

fn main() {
    let mut board: Board<10, 6, 5> = Board::new();

    loop {
        println!("{:?}'s turn:", board.turn);

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if let Ok(column) =  input.trim().parse() {
            match board.run(column) {
                Ok(Some(winner)) => {
                    println!("{}", board.grid);
                    println!("{winner:?} wins!");
                    break;
                },
                Ok(None) => println!("{}", board.grid),
                Err(error) => println!("{error}")
            }
        } else {
            println!("Please type a number");
        }
    }
}