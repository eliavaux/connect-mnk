#![allow(unused)]

mod board;
use board::*;

use std::io::stdin;
use std::num::ParseIntError;

fn main() {
    let mut board = Game::new(7, 6, 4);
    /*
    board.run(3);
    println!("{board}");
    println!("{:?}", board.last_score());
    board.run(4);
    println!("{board}");
    println!("{:?}", board.last_score());
    board.run(4);
    println!("{board}");
    println!("{:?}", board.last_score());
    board.run(3);
    println!("{board}");
    println!("{:?}", board.last_score());
    board.run(4);
    println!("{board}");
    println!("{:?}", board.last_score());
    board.run(3);
    println!("{board}");
    println!("{:?}", board.last_score());
    */

    
    // let p1 = Player::Computer(8);
    let p1 = Player::Human;
    let p2 = Player::Computer(10);
    // let p2 = Player::Human;
    
    play(&mut board, &p1, &p2);
   

    
    /* board.run(2);
    board.run(2);
    board.run(3);
    board.run(3);
    board.run(4);
    board.run(4);
   

    
    let move_order = [3, 2, 4, 1, 5, 0, 6];
    let move_order = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let (score, best_moves) = board.minimax_rec(10, &move_order);

    println!();
    println!("best_moves: {:?}", best_moves);
    println!("before");
    println!("{}", board);
    println!("{:?}", score);

    for column in best_moves {
        board.run(column);
    }
    
    println!("after");
    println!("{}", board);
    println!("{:?}", board.last_score()); */
   

    /*
    let a: Score = vec![-7, -1, 0, 1].into();
    let b: Score = vec![3, 0, 0, 0].into();

    dbg!(a.cmp(&b));
    dbg!(a > b);
    */
}

enum Player {
    Human,
    Computer(usize) // Search depth
}

fn play(board: &mut Game, p1: &Player, p2: &Player) {
    let move_order = [3, 2, 4, 1, 5, 0, 6];
    // let move_order = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    loop {
        let turn = board.turn();
        println!("{turn}'s turn");
        let player = match turn {
            Color::Red => p1,
            Color::Yellow => p2,  
        };
        match player {
            Player::Human => {
                match parse_input(board.width()) {
                    Ok(PlayerInput::Column(column)) => {
                        let state = board.run(column - 1);
                        println!("{board}");
                        println!("{:?}", board.last_score());
                        match state {
                            Ok(GameState::InProgress) => {},
                            Ok(GameState::Win(winner)) => {
                                println!("{winner} wins!");
                                break;
                            },
                            Ok(GameState::Draw) => {
                                println!("Draw!");
                                break;
                            },
                            Err(error) => {
                                match error {
                                    InsertError::InvalidColumn => println!("Column does not exist."),
                                    InsertError::ColumnFull => println!("Column is already full."),
                                }
                            }
                        }
                    },
                    Ok(PlayerInput::Undo) => {
                        board.undo();
                        println!("{board}");
                        println!("{:?}", board.last_score());
                    },
                    Ok(PlayerInput::Quit) => break,
                    Err(error) => println!("Could not parse input, try again")
                }
            }
            Player::Computer(depth) => {
                let (score, move_list) = board.minimax_rec(*depth, &move_order);
                let column = move_list.last().unwrap();
                let state = board.run(*column);
                println!("{board}");
                println!("{:?}", board.last_score());
                match state {
                    Ok(GameState::InProgress) => {},
                    Ok(GameState::Win(winner)) => {
                        println!("{winner} wins!");
                        break;
                    },
                    Ok(GameState::Draw) => {
                        println!("Draw!");
                        break;
                    },
                    Err(error) => {
                        match error {
                            InsertError::InvalidColumn => println!("Column does not exist."),
                            InsertError::ColumnFull => println!("Column is already full."),
                        }
                    }
                }
            }
        }
    }
}

enum PlayerInput {
    Column(usize),
    Undo,
    Quit
}

enum ParseInputError {
    Parse(ParseIntError),
    OutOfRange,
}

fn parse_input(columns: usize) -> Result<PlayerInput, ParseInputError> {
    println!("Enter a column number: ");
    let input = input();
    
    match input.parse() {
        Ok(column) => {
            if (1..=columns).contains(&column) {
                Ok(PlayerInput::Column(column))
            } else {
                Err(ParseInputError::OutOfRange)
            }
        },
        Err(error) => match input.as_str() {
            "u" | "undo" => Ok(PlayerInput::Undo),
            "q" | "quit" => Ok(PlayerInput::Quit),
            _ => Err(ParseInputError::Parse(error))
        }
    }
}

fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}
