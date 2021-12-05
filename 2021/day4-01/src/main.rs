// skeleton from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

use core::num;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::{marker, fmt};
use std::path::Path;
use std::process::exit;

fn comma_line_to_vec(line: &str) -> Vec<u32> {
    line.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>()
}

fn ws_line_to_vec(line: &str) -> Vec<u32> {
    let mut result = Vec::<u32>::new();
    for w in line.split(" ") {
        match w.parse::<u32>() {
            Ok(x) => result.push(x),
            Err(_) => {}, // skipping alignment spaces
        }
    }
    result
}

struct BingoBoard {
    number_matrix : Vec<Vec<u32>>,
    marker_matrix : Vec<Vec<bool>>,
    won : bool,
}

impl BingoBoard {
    fn from_lines(lines : & mut io::Lines<io::BufReader<File>>) -> Result<BingoBoard, u32> {
        let mut number_matrix: Vec<Vec<u32>> = Vec::new();
        let mut marker_matrix: Vec<Vec<bool>> = Vec::new();
        let won = false;
        loop {
            let single_line = match lines.next() {
                Some(s) => {s.unwrap()},
                None => { 
                    /* Two cases:
                       1. We hit EOF and read lines before -> return the last board.
                       2. We hit EOF without reading anything -> signal an error
                       */
                    if number_matrix.len() > 0 {
                        return Ok(BingoBoard { number_matrix, marker_matrix, won });
                    } else {
                        return Err(1)
                    }
                }
            };

            if single_line.len() > 0 {
                let nums = ws_line_to_vec(&single_line);
                let markers = vec![false; nums.len()];
                number_matrix.push(nums);
                marker_matrix.push(markers);
            } else {
                return Ok(BingoBoard { number_matrix, marker_matrix, won });
            }
        }
    }

    fn is_winning(&self) -> bool {
        for m in &self.marker_matrix {
            if !m.contains(&false) { // we have a line that is entirely 'true'
                return true;
            }
        }

        for i in 0..self.marker_matrix[0].len() {
            let mut vert = true;
            for m in &self.marker_matrix {
                if m[i] == false { vert = false; break; }
            }
            if vert { return true; }
        }
        false
    }

    fn mark(& mut self, num:u32) -> () {
        for row in 0..self.number_matrix.len() {
            for col in 0..self.number_matrix[0].len() {
                if self.number_matrix[row][col] == num {
                    self.marker_matrix[row][col] = true;
                }
            }

        }
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for row in 0..self.number_matrix.len() {
            for col in 0..self.number_matrix[0].len() {
                if !self.marker_matrix[row][col] {
                    sum += self.number_matrix[row][col];
                }
            }
        }
        sum
    }
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BOARD {:?}", self.number_matrix)
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    let mut values = Vec::<u32>::new();
    let mut boards = Vec::<BingoBoard>::new();
    if let Ok(mut lines) = read_lines("./input") {
        
        if let Ok(line)  = lines.next().unwrap() {
            values = comma_line_to_vec(&line);
        }
        println!("values: {:?}", values);

        if let Ok(skip_line) = lines.next().unwrap() {
            if skip_line.len() > 0 {
                println!("Input Error: expected empty line!");
                exit(1)
            }
        }

        loop {
            let b = match BingoBoard::from_lines(& mut lines) {
                Ok(b) => { b },
                Err(_) => { break },
            };
            
            println!("{}", b);
            boards.push(b);
        }

        println!("Found {} boards.", boards.len());
    }

    println!("Now making playing moves...");

    for num in values {
        let mut won_count = 0;
        let board_count = boards.len();
        for b in boards.iter() {
            if b.is_winning() { won_count += 1 }
        }

        println!("NUM {} (won so far: {}/{})", num, won_count, boards.len());

        for b in boards.iter_mut() {
            b.mark(num);
            if b.is_winning () && !b.won {
                if won_count == board_count - 1 {
                    println!("WIN! {}", b);
                    let su = b.sum_unmarked();
                    println!("   unmarked sum: {}", su);
                    println!("   SOLUTION = {}", su * num);
                    return;
                }
                b.won = true;
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
