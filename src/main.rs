extern crate rand;

use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

enum BufferLocation {Foreground, Background}

struct Gol {
    rows:     usize,
    cols:     usize,
    switch:   bool,
    buffer_a: Vec<Vec<bool>>,
    buffer_b: Vec<Vec<bool>>,
}

impl Gol {
    fn new(rows: usize, cols: usize) -> Gol {
        // create a 2-dimensional field with a phantom frame
        let buffer_a = vec![vec![false; cols + 2]; rows + 2];
        let buffer_b = buffer_a.clone();

        Gol {
            rows:     rows,
            cols:     cols,
            switch:   true,
            buffer_a: buffer_a,
            buffer_b: buffer_b,
        }
    }

    fn init(&mut self) {
        let (rows, cols) = (self.rows, self.cols);
        let mut buffer   = self.get_buffer(BufferLocation::Foreground);
        
        let mut rng      = rand::thread_rng();
        let between      = Range::new(0, 100);

        for row in 1..rows + 1 {
            for col in 1..cols + 1 {
                buffer[row][col] = between.ind_sample(&mut rng) < 5;
            }
        }
    }

    fn print(&mut self) {
        let (rows, cols) = (self.rows, self.cols);
        let buffer_fg    = self.get_buffer(BufferLocation::Foreground);

        for row in 1..rows + 1 {
            for col in 1..cols + 1 {
                if buffer_fg[row][col] { print!("•") } else { print!(" ") };
            }
            println!("");
        }
    }
  
    fn next(&mut self) {
        {
            let (rows, cols)  = (self.rows, self.cols);

            // TODO: We need an mutable handle to buffer_bg and an immutable handle to buffer_fg.
            // This is not possible due to the borrowing policies. What is an idiomatic way of doing this?
            let     buffer_fg = self.get_buffer(BufferLocation::Foreground).clone();
            let mut buffer_bg = self.get_buffer(BufferLocation::Background);

            for row in 1..rows + 1 {
                for col in 1..cols + 1 {
                    let mut counter = -1;
                    for row_d in 0..3 {
                        for col_d in 0..3 {
                            if buffer_fg[row + row_d - 1][col + col_d - 1] {
                                counter += 1;
                            }
                        }
                    }
                    buffer_bg[row][col] = counter == 2 || counter == 3;
                }
            }
        }

        self.switch_buffer();
    }

    fn get_buffer(&mut self, loc: BufferLocation) -> &mut Vec<Vec<bool>> {
        match loc {
            BufferLocation::Foreground => {
                match self.switch {
                    true  => &mut self.buffer_a,
                    false => &mut self.buffer_b,
                }
            },
            BufferLocation::Background => {
                match self.switch {
                    true  => &mut self.buffer_b,
                    false => &mut self.buffer_a,
                }
            }
        }
    }

    fn switch_buffer(&mut self) {
        self.switch = !self.switch;
    }
}

fn main() {
    let mut gol = Gol::new(40, 60);
    
    gol.init();

    loop {
        println!("------------------------------------------------------------");
        gol.print();
        println!("------------------------------------------------------------");
        gol.next();
        sleep(Duration::from_millis(100));
    }
}
