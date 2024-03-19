use std::io::stdin;
use crate::game::{Color, Game, Score};

pub mod game;

fn main() {
    let mut game: Game<7, 6, 4> = Game::new();

    // play(&mut game, (Some(12), Some(12)), [3,4,2,5,1,6,0]); // computer vs computer
    play(&mut game, (None, Some(12)), [3,4,2,5,1,6,0]); // player vs computer
    // play(&mut game, (None, None), [3,4,2,5,1,6,0]); // player vs player
}

fn play<const M: usize, const N: usize, const K: usize>(game: &mut Game<M,N,K>, computers: (Option<usize>, Option<usize>), order_list: [usize; M]) {
    let mut go = true;
    while go {
        println!("\n{:?}'s turn:", game.turn);
        if game.turn == Color::Red {
            if let Some(go_) = turn(game, computers.0, order_list) { go = go_ }
        } else if let Some(go_) = turn(game, computers.1, order_list) { go = go_ }
    }
}

fn turn<const M: usize, const N: usize, const K: usize>(game: &mut Game<M,N,K>, computer: Option<usize>, order_list: [usize; M]) -> Option <bool> {
    if let Some(depth) = computer { Some(computer_turn(game, depth, order_list)) }
    else { player_turn(game) }
}


fn player_turn<const M: usize, const N: usize, const K: usize>(game: &mut Game<M,N,K>) -> Option<bool> {
    let input = input();
    if input == "undo" { game.undo(); game.undo(); return None }

    if let Ok(column) =  input.parse::<usize>() {
        Some(turn_inner(game, column.saturating_sub(1)))
    } else { println!("Please type a number"); None }
}

fn computer_turn<const M: usize, const N: usize, const K: usize>(game: &mut Game<M,N,K>, depth: usize, order_list: [usize; M]) -> bool {
    println!("Computer is thinking...");
    let (eval, best_move) = game.minimax(depth, Score([-1;K]), Score([1;K]), &order_list);
    println!("Computer's evaluation: {eval:?}");
    turn_inner(game, best_move)
}

fn turn_inner<const M: usize, const N: usize, const K: usize>(game: &mut Game<M,N,K>, column: usize) -> bool {
    let result = game.run(column);

    if result.is_ok() {
        println!("{}", game.board);
        println!("Score: {:?}", game.last_score());
    }
    match result {
        Ok(Some(true)) => { println!("{:?} wins!", game.not_turn()); false },
        Ok(Some(false)) => { println!("Draw!"); false },
        Ok(None) => true,
        Err(error) => {println!("{error}"); true}
    }
}

fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}
