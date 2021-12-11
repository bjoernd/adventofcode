use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};
use clap::App;
use std::time::{Duration, Instant};

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

struct FishStats {
    all_the_fish: Vec<u64>,
}

impl FishStats {

    fn from_vec(input: Vec<u32>) -> FishStats {
        let mut gen_count = vec![0; 9];
        for i in input {
            gen_count[i as usize] += 1;
        }
        FishStats { all_the_fish: gen_count }
    }

    fn count(self : &FishStats) -> u64 {
        self.all_the_fish.iter().sum()
    }

    fn print(self: &FishStats) {
        let mut strings = vec![];
        for u in self.all_the_fish.iter() { strings.push(u.to_string()); }
        println!("{}", strings.join(","));
    }
}

fn simulate_fish_one_step(fish: &mut FishStats) {

    /* Fish with gen 0 are special and we treat them after all others... */
    let mut to_spawn = fish.all_the_fish[0];

    /* in all other generations, we just move them to the next gen */
    for i in 1..fish.all_the_fish.len() {
        fish.all_the_fish[i-1] = fish.all_the_fish[i];
    }

    /* Now we spawn: 1 fish for every to_spawn */
    fish.all_the_fish[8] = to_spawn;
    /* and the replicated fish reset to 6 */
    fish.all_the_fish[6] += to_spawn;
}

fn run(filename: &str) -> std::io::Result<()> {
    let mut fish = FishStats::from_vec(read_input(filename)?);
    println!(
        "Found {} items:",
        fish.count()
    );

    for i in 0..257 {
        print!("Day {:2} [{:5}] ", i, fish.count());
        fish.print();
        simulate_fish_one_step(&mut fish);
    }

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
