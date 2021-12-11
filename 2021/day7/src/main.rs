use std::{
    fs::File,
    io::{BufReader, Read},
};
use clap::App;
use std::time::{Instant};

fn read_input(filename: &str) -> std::io::Result<Vec<u32>> {
    println!("Specified file name {}", filename);
    let mut reader = BufReader::new(File::open(filename)?);

    let mut line = vec![];
    reader.read_to_end(&mut line)?;
    let line_str = String::from_utf8_lossy(&line);
    let mut data = Vec::<u32>::new();

    for item in line_str.split(",") {
        data.push(item.parse::<u32>().unwrap());
    }

    Ok(data)
}

fn evaluate_cost(d0: u32, d1: u32) -> u32 {
    let absdiff = (d0 as i32 - d1 as i32).abs() as u32;
    absdiff * (absdiff + 1) / 2
}

fn cost(data: &Vec<u32>, pos: u32) -> u32 {
    let mut res = 0;

    for i in 0..data.len() {
        res += evaluate_cost(data[i], pos);
    }

    res as u32
}

fn run(filename: &str) -> std::io::Result<()> {

    let data = read_input(filename)?;
    
    let minimum = data.iter().min().unwrap();
    let maximum = data.iter().max().unwrap();

    println!("Items: {} Max: {} Min: {}", data.len(), maximum, minimum);

    let mut min_c = u32::MAX;
    let mut min_pos = u32::MAX;

    for i in 0..*maximum {
        let c = cost(&data, i);

        if c < min_c {
            min_c = c;
            min_pos = i;
        }

        println!("Target {:2} -> cost {:3}", i, cost(&data, i));
    }

    println!("Minimum pos is {} with cost {}", min_pos, min_c);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let matches = App::new("day5")
        .version("1.0.")
        .author("Bjoern Doebel <bjoern.doebel@gmail.com")
        .about("Advent of Code 2021, day 6")
        .arg("<FILE> 'Specify input file'")
        .get_matches();

    if let Some(filename) = matches.value_of("FILE") {
        let start = Instant::now();
        let x = run(filename);
        let duration = start.elapsed();
        println!("Completed in {:?}", duration);
        return x
    }

    Ok(())
}
