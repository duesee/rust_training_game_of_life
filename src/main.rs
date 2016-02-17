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
    // Instead of using a boolean variable, we encode the cells with an
    // non negative integer. The cell is alive iff the int is equal to
    // 255. This will be used to implement a "fade-out effect".
    buffer_a: Vec<Vec<u8>>,
    buffer_b: Vec<Vec<u8>>,
}

impl Board {
    fn new(rows: usize, cols: usize) -> Board {
        Board {
            rows:     rows,
            cols:     cols,
            switch:   true,
            // Allocate two more rows and columns to construct an always
            // dead phantom border. This simplifies the algorithm.
            buffer_a: vec![vec![0; cols + 2]; rows + 2],
            buffer_b: vec![vec![0; cols + 2]; rows + 2],
        }
    }

    fn population_from_rand(&mut self) {
        let &mut Board { rows, cols, ref mut buffer_a, .. } = self;
        
        let mut rng = rand::thread_rng();
        let between = rand::distributions::Range::new(0, 100);

        for row in 1..rows + 1 {
            for col in 1..cols + 1 {
                // 15 out of 100 cells should be alive.
                if between.ind_sample(&mut rng) < 15 {
                    buffer_a[row][col] = 255;
                }
            }
        }
    }

    #[allow(dead_code)]
    fn population_from_vec(&mut self, data: Vec<(usize, usize)>) {
        for (row, col) in data {
            if row < self.rows && col < self.cols {
                self.buffer_a[row + 1][col + 1] = 255;
            } else {
                panic!("cell not in field");
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
                let mut counter = 0;
                let cell_fg     = &buffer_fg[row][col];
                let alive       = *cell_fg == 255;
                
                // Start with minus 1 if the cell is alive to eliminate handling a
                // special case, i.e. the middle cell, when counting the neighbors.
                if  alive { counter = -1 }
                // How many neighbors are alive?
                for row_d in 0..3 {
                    for col_d in 0..3 {
                        if buffer_fg[row + row_d - 1]
                                    [col + col_d - 1] == 255 {
                            counter += 1;
                        }
                    }
                }
                if alive {
                    if counter == 2 || counter == 3 {
                        buffer_bg[row][col] = 255;
                    } else if *cell_fg >= 1 {
                        // Instead of setting to false, decrement by one.
                        // This allows a cool fade-out effect.
                        buffer_bg[row][col] = *cell_fg - 1;
                    }
                } else {
                    if counter == 3 {
                        buffer_bg[row][col] = 255;
                    } else if *cell_fg >= 1 {
                        // Instead of setting to false, decrement by one.
                        // This allows a cool fade-out effect.
                        buffer_bg[row][col] = *cell_fg - 1;
                    }
                }
            }
        }
        
        // Don't forget to swap the buffers.
        self.switch = !self.switch;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buffer_fg = match self.switch {
            true  => &self.buffer_a,
            false => &self.buffer_b
        };

        for row in 1..self.rows + 1 {
            for col in 1..self.cols + 1 {
                // Which symbols should be used for the fade-out?
                match buffer_fg[row][col] {
                    255       => try!(write!(f, "●")), // cell is alive
                    253...255 => try!(write!(f, "◍")), // cell died <= 2 gens ago
                    249...253 => try!(write!(f, "○")), // cell died <= 4 gens ago
                    241...249 => try!(write!(f, "◌")), // cell died <= 8 gens ago
                    _         => try!(write!(f, " ")), // cell died  > 8 gens ago
                }

            }
            try!(writeln!(f, ""));
        }
        
        Ok(())
    }
}

fn main() {
    let mut board = Board::new(60, 80);

    // Random.
    board.population_from_rand();
    
    // Glider.
    //board.population_from_vec(vec![
    //    (10, 10), (11, 11), (12,  9), (12, 10), (12, 11)
    //]);
    
    // Gosper Glider Gun.
    //board.population_from_vec(vec![
    //    (2, 26), (3, 24), (3, 26), (4, 14), ( 4, 15), ( 4, 22),
    //    (4, 23), (4, 36), (4, 37), (5, 13), ( 5, 17), ( 5, 22),
    //    (5, 23), (5, 36), (5, 37), (6,  2), ( 6,  3), ( 6, 12),
    //    (6, 18), (6, 22), (6, 23), (7,  2), ( 7,  3), ( 7, 12),
    //    (7, 16), (7, 18), (7, 19), (7, 24), ( 8, 12), ( 8, 18),
    //    (7, 26), (9, 13), (9, 17), (8, 26), (10, 14), (10, 15),
    //]);

    for cnt in 1.. {
        // Formatting is explained here:
        // https://doc.rust-lang.org/std/fmt/
        println!("{1}\n{2:^0$}", board.cols,
                                 board,
                                 format!("<<< Generation: {:>6} >>>", cnt));
        board.next();
        sleep(Duration::from_millis(125));
    }
}
