use std::io::stdin;
use crate::game::Game;

pub mod game;

fn main() {
    two_players();
}

// fn minimax() { todo!() }

fn two_players() {
    let mut game: Game<7, 6, 4> = Game::new();

    loop {
        println!("{:?}'s turn:", game.turn);

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim() == "undo" { game.undo(); continue }

        if let Ok(column) =  input.trim().parse::<usize>() {
            match game.run(column.saturating_sub(1)) {
                Ok(Some(true)) => { println!("{} \n{:?} wins!", game.board, /* wrong player */game.turn); break },
                Ok(Some(false)) => { println!("{} \nDraw!", game.board); break },
                Ok(None) => println!("{}", game.board),
                Err(error) => println!("{error}")
            }

        } else {
            println!("Please type a number");
        }

        println!("Red {:?}", game.score.0);
        println!("Yellow {:?}", game.score.1);
        println!();
    }
}
