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


        println!("Red: max row: {}, number of max rows: {}", game.score.0.max_row, game.score.0.num_max_rows);
        println!("Yellow: max row: {}, number of max rows: {}",game.score.1.max_row, game.score.1.num_max_rows);
        println!();
    }
}