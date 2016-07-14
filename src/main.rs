extern crate docopt;
extern crate rustc_serialize;
extern crate rust_training_game_of_life;

use docopt::Docopt;
use rust_training_game_of_life::Board;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

const USAGE: &'static str = " 
Game of Life.

Usage:
  gol random <rows> <columns>
  gol glider <rows> <columns>
  gol gosper <rows> <columns>
  gol (-h | --help)

Options:
  -h --help     Show this screen.
  
";

#[derive(RustcDecodable)]
struct Args {
    arg_rows:     usize,
    arg_columns:  usize,
    cmd_random:   bool,
    cmd_glider:   bool,
    cmd_gosper:   bool,
}

type CliResult = Result<(), Box<Error>>;

/// Populate randomly.
fn example1(board: &mut Board) -> CliResult {
    Ok(board.population_from_rand())
}

/// Populate with a glider.
fn example2(board: &mut Board) -> CliResult {
    Ok(try!(board.population_from_vec(vec![
        (10, 10), (11, 11), (12,  9), (12, 10), (12, 11)
    ])))
}

/// Populate with a gosper glider gun.
fn example3(board: &mut Board) -> CliResult {
    Ok(try!(board.population_from_vec(vec![
        (2, 26), (3, 24), (3, 26), (4, 14), ( 4, 15), ( 4, 22),
        (4, 23), (4, 36), (4, 37), (5, 13), ( 5, 17), ( 5, 22),
        (5, 23), (5, 36), (5, 37), (6,  2), ( 6,  3), ( 6, 12),
        (6, 18), (6, 22), (6, 23), (7,  2), ( 7,  3), ( 7, 12),
        (7, 16), (7, 18), (7, 19), (7, 24), ( 8, 12), ( 8, 18),
        (7, 26), (9, 13), (9, 17), (8, 26), (10, 14), (10, 15),
    ])))
}

fn run(args: &Args) -> CliResult {                           
    let mut board = Board::new(args.arg_rows, args.arg_columns);

    if args.cmd_random {
        try!(example1(&mut board))
    } else if args.cmd_glider {
        try!(example2(&mut board))
    } else if args.cmd_gosper {
        try!(example3(&mut board))
    } else {
        unreachable!()
    }
    
    for cnt in 1.. {
        // Formatting strings are explained here: https://doc.rust-lang.org/std/fmt/
        println!("{1}\n{2:^0$}", args.arg_columns, board, format!("<<< Generation: {:>6} >>>", cnt));
        board.next();
        sleep(Duration::from_millis(100));
    }
    
    Ok(())
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    match run(&args) {
        Ok(_)    => (),
        Err(err) => { println!("Something went wrong: {}", err.description()); return }
    }
}
