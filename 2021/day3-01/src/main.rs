// skeleton from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        let mut ones = vec![0; 12];
        let mut count = 0;
        for line in lines {
            if let Ok(numstring) = line {
                let intval = isize::from_str_radix(numstring.as_str(), 2).unwrap();
                //println!("{}", intval);
                for i in 0..12 {
                    if intval & (1 << i) != 0 {
                        ones[i] += 1;
                    }
                }
                count += 1;
            }
        }
        println!("Counter: {}", count);

        let mut gamma = 0;
        let mut epsilon = 0;
        for i in 0..12 {
            println!("vec[{}] = {}", i, ones[i]);
            let mut mcb = 0;
            let mut lcb = 0;
            if ones[i] > count/2 { // ones are the MCB
                mcb = 1;
                lcb = 0;
            } else { // ones are the LCB
                mcb = 0;
                lcb = 1;
            }
            gamma |= (mcb << i);
            epsilon |= (lcb << i);
        }
        println!("gamma {} epsilon {} product {}", gamma, epsilon, gamma*epsilon);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
