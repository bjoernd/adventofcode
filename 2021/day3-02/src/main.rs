// skeleton from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_IDX :u32 = 12;

fn bit_is_1(number :i32, position: u32) -> bool {
    number & (1 << position) > 0
}

fn bit_is_0(number :i32, position: u32) -> bool {
    number & (1 << position) == 0
}

fn filter_most_common(values: &Vec<i32>, position: u32) -> Vec<i32> {
    let ones: Vec<&i32> = values.into_iter().filter(|x| bit_is_1(**x, position)).collect::<Vec<&i32>>();

    let mut result = Vec::new();

    let one_count = ones.len();
    let zero_count = values.len() - one_count;

    //println!("        MC: Ones len {} values len {} threshold {}", ones.len(), values.len(), values.len()/2);
    if one_count >= zero_count {
        for o in ones {
            //println!("        1 {:b}", *o);
            result.push(*o);
        }
    } else {
        for o in values.into_iter().filter(|x| bit_is_0(**x, position)).collect::<Vec<&i32>>() {
            //println!("        0 {:b}", *o);
            result.push(*o);
        }
    }

    result
}

fn filter_least_common(values: &Vec<i32>, position: u32) -> Vec<i32> {
    let ones: Vec<&i32> = values.into_iter().filter(|x| bit_is_1(**x, position)).collect::<Vec<&i32>>();

    let mut result = Vec::new();

    let one_count = ones.len();
    let zero_count = values.len() - one_count;

    //println!("        LC: Ones len {} values len {} threshold {}", ones.len(), values.len(), values.len()/2);
    if zero_count <= one_count {
        for o in values.into_iter().filter(|x| bit_is_0(**x, position)).collect::<Vec<&i32>>() {
            //println!("        0 {:05b}", *o);
            result.push(*o);
        }
    } else {
        for o in ones {
            //println!("        1 {:05b}", *o);
            result.push(*o);
        }
    }

    result
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        let mut values = Vec::new();
        for line in lines {
            if let Ok(numstring) = line {
                let intval = i32::from_str_radix(numstring.as_str(), 2).unwrap();
                values.push(intval);
            }
        }
        println!("Value count: {}", values.len());

        let mut idx = MAX_IDX;

        println!("Filtering by most common bit...");
        let mut most_common = values.clone();
        while most_common.len() > 1 {
            idx -= 1;
            most_common = filter_most_common(&most_common, idx);
            println!("   [{}] remaining mc: {}", idx, most_common.len());
        }

        println!("Filtering by least common bit...");
        idx = MAX_IDX;
        let mut least_common = values.clone();
        while least_common.len() > 1 {
            idx -= 1;
            least_common = filter_least_common(&least_common, idx);
            println!("   [{}] remaining lc: {}", idx, least_common.len());
        }

        println!("mc {}", most_common[0]);
        println!("lc {}", least_common[0]);
        println!("result {}", most_common[0] * least_common[0]);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
