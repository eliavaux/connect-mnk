use std::io::stdin;
use crate::game::Game;

pub mod game;

fn main() {
    let mut game: Game<7, 6, 4> = Game::new();

    loop {
        println!("{:?}'s turn:", game.turn);

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if let Ok(column) =  input.trim().parse() {
            match game.run(column) {
                Ok(Some(winner)) => {
                    println!("{}", game.board);
                    println!("{winner:?} wins!");
                    break;
                },
                Ok(None) => println!("{}", game.board),
                Err(error) => println!("{error}")
            }
        } else {
            println!("Please type a number");
        }
    }
}