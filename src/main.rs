extern crate rust_training_game_of_life;

use rust_training_game_of_life::Board;
use std::error::Error;
use std::io::{self, stdout};
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

fn read_usize(msg: &str) -> Result<usize, Box<Error>> {
    print!("{}", msg);
    try!(stdout().flush());
    
    let mut input = String::new();
    try!(io::stdin().read_line(&mut input));

    // parse::<usize> will be inferred from the context.
    Ok(try!(input.trim().parse()))
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

fn goodbye(err: Box<Error>) -> usize {
    println!("************************************************************");
    println!("Oh no: {}", err.description());
    println!("************************************************************");
    std::process::exit(-1);
}

fn main() {
    let rows = read_usize("How many rows (20...100): ").unwrap_or_else(&goodbye);
    match rows {
        x @ 20...100 => x,
        _            => panic!("Please choose a row count between 20 and 100.")
    };

    let cols = read_usize("How many columns (40...100): ").unwrap_or_else(&goodbye);
    match cols {
        x @ 40...100 => x,
        _            => panic!("Please choose a column count between 40 and 100.")
    };
    
    let fig = read_usize("Choose \n\t0) Random\n\t1) Glider\n\t2) Gosper Glider Gun\nas figure: ").unwrap_or_else(&goodbye);
    match fig {
        x @ 0...2 => x,
        _         => panic!("Please choose an existing population.")
    };

    let mut board = Board::new(rows, cols);
    
    // Vector of functions.
    let examples: Vec<fn(&mut Board)> = vec![example1,
                                             example2,
                                             example3];
    
    examples[fig](&mut board);

    for cnt in 1.. {
        // Formatting strings are explained here: https://doc.rust-lang.org/std/fmt/
        println!("{1}\n{2:^0$}", cols, board, format!("<<< Generation: {:>6} >>>", cnt));
        board.next();
        sleep(Duration::from_millis(100));
    }
}
