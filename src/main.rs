pub mod game;

use crate::game::Game;


fn main() {
    start_game();
}

fn start_game() {
    let mut game = Game::default();
    let mut is_game = true;

    println!("Type the column you want to insert a chip in (1-{})", game::COLS);
    println!("Type 'quit' to quit");

    while is_game {
        let game = game.run();

        if let Err(e) = game {
            eprintln!("err: {e}");
        } else {
            is_game = game.unwrap();
        }
    }
}
