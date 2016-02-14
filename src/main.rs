extern crate rand;

use rand::distributions::IndependentSample;
use std::fmt;
use std::thread::sleep;
use std::time::Duration;

struct Board {
    rows:     usize,
    cols:     usize,
    // We use two buffers: During calculation, the foreground buffer is
    // read-only and changes to the cells are written to the background
    // buffer. When a calculation has finished, the buffers are swapped,
    // so that the fg-buffer becomes the bg-buffer and vice versa.
    switch:   bool,
    buffer_a: Vec<Vec<bool>>,
    buffer_b: Vec<Vec<bool>>,
}

impl Board {
    fn new(rows: usize, cols: usize) -> Board {
        Board {
            rows:     rows,
            cols:     cols,
            switch:   true,
            // Allocate two more rows and columns to construct an always
            // dead phantom border. This simplifies the algorithm.
            buffer_a: vec![vec![false; cols + 2]; rows + 2],
            buffer_b: vec![vec![false; cols + 2]; rows + 2],
        }
    }

    fn populate(&mut self) {
        let &mut Board { rows, cols, ref mut buffer_a, .. } = self;
        
        let mut rng = rand::thread_rng();
        let between = rand::distributions::Range::new(0, 100);

        for row in 1..rows + 1 {
            for col in 1..cols + 1 {
                // 7 out of 100 cells should be alive.
                buffer_a[row][col] = between.ind_sample(&mut rng) < 7;
            }
        }
    }

    fn next(&mut self) {
        let &mut Board { rows, cols, switch, ref mut buffer_a, ref mut buffer_b } = self;

        let (buffer_fg, buffer_bg) = match switch {
            true  => (buffer_a, buffer_b),
            false => (buffer_b, buffer_a),
        };

        for row in 1..rows + 1 {
            for col in 1..cols + 1 {
                // Another small simplification: Instead of starting
                // the counter with zero, start with minus 1 ...
                let mut counter = -1;
                for row_d in 0..3 {
                    for col_d in 0..3 {
                        if buffer_fg[row + row_d - 1]
                                    [col + col_d - 1] {
                            counter += 1;
                        }
                    }
                }
                // ... and adapt the rules accordingly. 
                buffer_bg[row][col] = counter == 2 || counter == 3;
            }
        }
        
        // Don't forget to swap the buffers.
        self.switch = !self.switch;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (rows, cols) = (self.rows, self.cols);
        let buffer_fg    = match self.switch {
            true  => &self.buffer_a,
            false => &self.buffer_b
        };

        for row in 1..rows + 1 {
            for col in 1..cols + 1 {
                if buffer_fg[row][col] {
                    try!(write!(f, "•"))
                } else {
                    try!(write!(f, " "))
                };
            }
            try!(writeln!(f, ""));
        }
        
        Ok(())
    }
}

fn main() {
    let mut board = Board::new(40, 60);
    
    board.populate();

    for cnt in 1.. {
        board.next();
        println!("{}", board);
        println!("{:^60}", format!("<<< Generation: {:>6} >>>", cnt));
        sleep(Duration::from_millis(100));
    }
}
