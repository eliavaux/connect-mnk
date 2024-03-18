use std::cmp::Ordering;
use std::io::stdin;
use crate::game::{Color, Game, Score};

pub mod game;

fn main() {
    let mut game: Game<7, 6, 4> = Game::new();

    // no_player(&mut game, [3,4,2,5,1,6,0]);
    one_player(&mut game, [3,4,2,5,1,6,0])
    // two_players(&mut game);
}

fn no_player<const M: usize, const N: usize, const K: usize>(game: &mut Game<M,N,K>, order_list: [usize; M]) {
    let score = game.minimax(4,  Score([-1;K]), Score([1;K]), &order_list);
    println!("score: {:?}", score);
}

fn one_player<const M: usize, const N: usize, const K: usize>(game: &mut Game<M, N, K>, order_list: [usize; M]) {
    loop {
        println!("\n{:?}'s turn:", game.turn);
        let result;

        if game.turn == Color::Red {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() == "undo" { game.undo(); continue }

            if let Ok(column) =  input.trim().parse::<usize>() {
                result = game.run(column.saturating_sub(1));
            } else { println!("Please type a number"); continue; }
        } else {
            let mut best_eval = Score([1; K]);
            let mut best_move = 0;

            for column in 0..M {
                if game.run(column) == Ok(None) {
                    let eval = game.minimax(10, Score([-1;K]), Score([1;K]), &order_list);
                    if best_eval.cmp(&eval) == Ordering::Greater {
                        best_eval = eval;
                        best_move = column;
                    }
                    game.undo();
                }
            }
            result = game.run(best_move);
        }
        println!("{}", game.board);
        println!("Score: {:?}", game.last_score());
        match result {
            Ok(Some(true)) => { println!("{:?} wins!", game.not_turn()); break },
            Ok(Some(false)) => { println!("Draw!"); break },
            Ok(None) => (),
            Err(error) => println!("{error}")
        }
    }
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
