// skeleton from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        let mut depth = 0;
        let mut position = 0;
        for line in lines {
            if let Ok(command) = line {
                let v: Vec<&str> = command.split(' ').collect();
                let cmd = v[0];
                let arg = v[1].parse::<i32>().unwrap();
                match cmd {
                    "forward" => { position += arg },
                    "up" => { depth -= arg },
                    "down" => { depth += arg },
                    &_ => { println!("E: invalid cmd {} with argument {}", cmd, arg); }
                }
            }
        }
        println!("depth: {} position: {} product: {}",
                 depth, position, depth * position);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
