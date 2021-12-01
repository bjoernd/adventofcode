// skeleton from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        let mut num_count = 0;
        let mut last_num = 0;
        let mut result = 0;
        for line in lines {
            if let Ok(num_str) = line {
                let num = num_str.parse::<i32>().unwrap();
                if num_count == 0 {
                    last_num = num;
                } else {
                    if num > last_num { result += 1; }
                    last_num = num;
                }
            }
            num_count += 1;
        }
        println!("Lines: {} Result: {}", num_count, result);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
