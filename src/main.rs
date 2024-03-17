use std::io::stdin;
use crate::game::{Game, Score};

pub mod game;

fn main() {
    let mut game: Game<7, 6, 4> = Game::new();

    // game.run(0);
    // game.run(0);
    // game.run(1);
    // game.run(1);
    // game.run(2);
    // println!("{}", game.board);

    // let (score_red, score_yellow) = game.last_score();
    // println!("red: {score_red:?}, yellow: {score_yellow:?}");
    // let red_greater_than = score_red > score_yellow;
    // println!("{red_greater_than}");

    no_player(&mut game, [3,2,4,1,5,0,6]);
    // two_players(&mut game);
}

fn no_player<const M: usize, const N: usize, const K: usize>(game: &mut Game<M,N,K>, order_list: [usize; M]) {
    let (score, moves) = game.minimax(8,  Score([-1;K]), Score([1;K]), order_list);
    println!("score: {:?}", score);
    println!("moves: {:?}", moves);
}

fn _two_players<const M: usize, const N: usize, const K: usize>(game: &mut Game<M,N,K>) {
    loop {
        println!("\n{:?}'s turn:", game.turn);

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() == "undo" { game.undo(); continue }

        if let Ok(column) =  input.trim().parse::<usize>() {
            let result = game.run(column.saturating_sub(1));
            println!("{}", game.board);
            println!("Score: {:?}", game.last_score());
            match result {
                Ok(Some(true)) => { println!("{:?} wins!", game.not_turn()); break },
                Ok(Some(false)) => { println!("Draw!"); break },
                Ok(None) => (),
                Err(error) => println!("{error}")
            }
        } else { println!("Please type a number") }
    }
}
