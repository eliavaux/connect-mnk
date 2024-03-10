use std::io::stdin;
use crate::game::Game;

pub mod game;

fn main() {
    let mut game: Game<7, 6, 4> = Game::new();

    two_players(&mut game);
}

fn minimax<const M: usize, const N: usize, const K: usize>(game: &Game<M,N,K>) {
    game.negamax();
}

fn two_players<const M: usize, const N: usize, const K: usize>(game: &mut Game<M,N,K>) {
    loop {
        println!("{:?}'s turn:", game.turn);

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim() == "undo" { game.undo(); continue }

        if let Ok(column) =  input.trim().parse::<usize>() {
            match game.run(column.saturating_sub(1)) {
                Ok(Some(true)) => { print_game(game); println!("{:?} wins!", game.turn); break },
                Ok(Some(false)) => { print_game(game); println!("Draw!"); break },
                Ok(None) => print_game(game),
                Err(error) => println!("{error}\n")
            }

        } else { println!("Please type a number\n") }
    }
}

fn print_game<const M: usize, const N: usize, const K: usize>(game: &Game<M,N,K>){
    println!("{}", game.board);
    println!();
    println!();
}
