use std::io::stdin;
use crate::game::Game;

pub mod game;

fn main() {
    let mut game: Game<7, 6, 4> = Game::new();

    two_players(&mut game);
}

fn _one_player<const M: usize, const N: usize, const K: usize>(game: &mut Game<M,N,K>) {
    game.negamax();
}

fn two_players<const M: usize, const N: usize, const K: usize>(game: &mut Game<M,N,K>) {
    loop {
        println!("\n{:?}'s turn:", game.turn);

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() == "undo" { game.undo(); continue }

        if let Ok(column) =  input.trim().parse::<usize>() {
            let result = game.run(column.saturating_sub(1));
            println!("{}", game.board);
            println!("Red: {:?}, Yellow: {:?}", game.score_list.0.0.last().unwrap(), game.score_list.1.0.last().unwrap());
            match result {
                Ok(Some(true)) => { println!("{:?} wins!", game.turn); break },
                Ok(Some(false)) => { println!("Draw!"); break },
                Ok(None) => (),
                Err(error) => println!("{error}")
            }
        } else { println!("Please type a number") }
    }
}
