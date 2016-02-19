extern crate rust_training_game_of_life;

use rust_training_game_of_life::Board;
use std::io::{self, stdout};
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

fn read_usize(msg: String) -> usize {
    print!("{}", msg);
    stdout().flush().ok().expect("Could not flush stdout.");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line...");

    // parse::<usize> will be inferred from the context.
    match input.trim().parse() {
        Ok(num) => num,
        Err(_)  => panic!("Couldn't parse input to u64..."),
    }
}

/// Populate randomly.
fn example1(board: &mut Board) {
    board.population_from_rand();
}

/// Populate with a Glider.
fn example2(board: &mut Board) {
    board.population_from_vec(vec![
        (10, 10), (11, 11), (12,  9), (12, 10), (12, 11)
    ]);
}

/// Populate with a Gosper Glider Gun.
fn example3(board: &mut Board) {
    board.population_from_vec(vec![
        (2, 26), (3, 24), (3, 26), (4, 14), ( 4, 15), ( 4, 22),
        (4, 23), (4, 36), (4, 37), (5, 13), ( 5, 17), ( 5, 22),
        (5, 23), (5, 36), (5, 37), (6,  2), ( 6,  3), ( 6, 12),
        (6, 18), (6, 22), (6, 23), (7,  2), ( 7,  3), ( 7, 12),
        (7, 16), (7, 18), (7, 19), (7, 24), ( 8, 12), ( 8, 18),
        (7, 26), (9, 13), (9, 17), (8, 26), (10, 14), (10, 15),
    ]);
}

fn main() {
    let rows = match read_usize(String::from("How many rows (1...100): ")) {
        x @ 1...100 => x,
        _           => panic!("Wrong row count...")
    };

    let cols = match read_usize(String::from("How many cols (1...100): ")) {
        x @ 1...100 => x,
        _           => panic!("Wrong col count")
    };
    
    let selection = match read_usize(String::from("Choose a population\n\t0) Random\n\t1) Glider\n\t2) Gosper Glider Gun\n--> ")) {
        x @ 0...2 => x,
        _         => panic!("Wrong selection...")
    };

    let mut board = Board::new(rows, cols);
    
    // Vector of functions.
    let examples: Vec<fn(&mut Board)> = vec![example1,
                                             example2,
                                             example3];
    
    examples[selection](&mut board);

    for cnt in 1.. {
        // Formatting strings are explained here:
        // https://doc.rust-lang.org/std/fmt/
        println!("{1}\n{2:^0$}", cols,
                                 board,
                                 format!("<<< Generation: {:>6} >>>", cnt));
        board.next();
        sleep(Duration::from_millis(100));
    }
}
