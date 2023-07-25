#[macro_use]
extern crate lazy_static;

use std::sync::RwLock;
use std::{env, io};
use rand::seq::SliceRandom;
mod board;

use board::Board;
use board::Move;

const TARGET_SCORE: i32 = 31;
const DEBUG_NUM_POSITIONS: bool = true;

const BOARD_REPR: &[&str] = &[
    "  ooo  ",
    "  ooo  ",
    "ooooooo",
    "oooxooo",
    "ooooooo",
    "  ooo  ",
    "  ooo  "
];
/*
  Other variations you can try:

    "European board"
    "  ooo  ",
    " ooooo ",
    "ooooooo",
    "oooxooo",
    "ooooooo",
    " ooooo ",
    "  ooo  "

    "Big board" (change board::BOARD_SIZE to 9)

    "   ooo   ",
    "   ooo   ",
    "   ooo   ",
    "ooooooooo",
    "ooooxoooo",
    "ooooooooo",
    "   ooo   ",
    "   ooo   ",
    "   ooo   "
 */

fn main() {

    env::set_var("RUST_BACKTRACE", "1");

    let mut board = Board::from_strings(BOARD_REPR);
    let stdin = io::stdin();

    println!("Welcome to senku! What do you want to do? Enter 1-4: ");
    
    loop {
        println!("1. Play a game");
        println!("2. Solve the game using a randomized search (100k rounds)");
        println!("3. Solve the game using a backtracking search");
        println!("4. Quit the program");

        let mut user_input = String::new();
        stdin.read_line(&mut user_input).unwrap();
        let choice = user_input.chars().next().expect("Input shouldn't be empty");
        match choice {
            '1' => game(),
            '2' => autoplay(),
            '3' => {
                backtrack(&mut board);
                println!("Found a solution with {} moves, here they are: {:?}", board.score, board.moves);
                println!("The final state of the board: \n {}", board.to_string());
            },
            '4' => break,
            _ => println!("Incorrect digit. ")
        };
    }
}

fn game() {
    let stdin = io::stdin();
    let mut board = Board::from_strings(BOARD_REPR);
    loop {
        println!("{}", board.to_string());
        println!("Enter your move or q to quit: ");
        println!("Valid moves: {:?}", board.valid_moves());

        let mut user_input = String::new();
        stdin.read_line(&mut user_input).expect("What?");
        //println!("The input length was {}", user_input.len());

        if user_input.starts_with('q') {
            println!("Thanks for playing!");
            break;
        }

        let mov = Move::parse_move(&user_input);
        if let Err(message) = mov {
            println!("{}", message);
            continue;
        }

        let mov = mov.expect("The input should be valid because we checked it earlier");
        if !board.is_move_valid(mov) {
            println!("That move is illegal!");
            continue;
        }

        board.make_move(mov);
    }
}

fn autoplay() {

    let num_rounds = 1000000;
    let mut best_board = Board::default();

    for round in 0..num_rounds {
        let board = autoplay_round();
        if board.score > best_board.score {
            println!("Found a new best solution at round {} with score {}!", round, board.score);
            best_board = board;
        }
        if round % 10000 == 0 {
            println!("{} rounds!", round);
        }
    }
    
    println!("Found a solution with {} moves, here they are: {:?}", best_board.score, best_board.moves);
    println!("The final state of the board: \n {}", best_board.to_string());
}

fn autoplay_round() -> Board {
    let mut board = Board::from_strings(BOARD_REPR);
    loop {
        let legal_moves = board.valid_moves();
        if legal_moves.is_empty() {
            break;
        }
        
        let random_move = *legal_moves.choose(&mut rand::thread_rng()).expect("At this point the list should not be empty");
        board.make_move(random_move);
    }
    board
}

lazy_static! {
    static ref POSITIONS: RwLock<i32> = RwLock::new(0);
}

/// returns whether the search was successful
fn backtrack(board: &mut Board) -> bool {

    if DEBUG_NUM_POSITIONS {
        let positions = POSITIONS.read().unwrap();
        if *positions % 200000 == 0 {
            println!("{} positions searched!", *positions);
            println!("{}", board.to_string());
        }
        drop(positions);
        *POSITIONS.write().unwrap() += 1;
    }

    if board.score >= TARGET_SCORE {
        return true;
    }
    let legal_moves = board.valid_moves();

    for mov in legal_moves {
        board.make_move(mov);
        if backtrack(board) {
            return true;
        }
        // backtrack
        board.undo_move(mov);
    }
    false
}

