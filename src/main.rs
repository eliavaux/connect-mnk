pub mod game;

use crate::game::Game;


fn main() {
    start_game();
}

fn start_game() {
    let mut game = Game::new();
    let mut game_end = false;

    println!("Type the column you want to insert a chip in (1-{})", game::COLS);
    println!("Type 'quit' to quit");

    while !game_end {
        let game = game.run();
        if let Err(e) = game {
            eprintln!("err: {e}");
        } else {
            game_end = game.unwrap();
        }
    }
}
