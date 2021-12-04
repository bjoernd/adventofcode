// skeleton from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

use core::num;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::marker;
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
}

impl BingoBoard {
    fn from_lines(lines : & mut io::Lines<io::BufReader<File>>) -> Result<BingoBoard, u32> {
        let mut number_matrix: Vec<Vec<u32>> = Vec::new();
        let mut marker_matrix: Vec<Vec<bool>> = Vec::new();
        loop {
            let single_line = match lines.next() {
                Some(s) => {s.unwrap()},
                None => { 
                    /* Two cases:
                       1. We hit EOF and read lines before -> return the last board.
                       2. We hit EOF without reading anything -> signal an error
                       */
                    if number_matrix.len() > 0 {
                        return Ok(BingoBoard { number_matrix, marker_matrix });
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
                return Ok(BingoBoard { number_matrix, marker_matrix });
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
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(mut lines) = read_lines("./input") {
        let mut values = Vec::<u32>::new();
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

        let mut boards = Vec::<BingoBoard>::new();

        loop {
            let b = match BingoBoard::from_lines(& mut lines) {
                Ok(b) => { println!("{:?}", b.number_matrix); b },
                Err(_) => { break },
            };
            
            if b.is_winning() {
                println!("Winning board: {:?}", &b.number_matrix);
            }

            boards.push(b);
        }

        println!("Found {} boards.", boards.len());

    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
