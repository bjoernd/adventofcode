use std::{
    fs::File,
    io::{BufRead, BufReader},
};
#[macro_use]
extern crate scan_fmt;

use clap::App;
use scan_fmt::parse::ScanError;

struct Line {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

impl Line {
    fn from_string(line: &str) -> std::result::Result<Line, ScanError> {
        let (mut start_x, mut start_y, mut end_x, mut end_y) =
            scan_fmt!(line, "{},{} -> {},{}", usize, usize, usize, usize)?;

        Ok(Line {
            start_x,
            start_y,
            end_x,
            end_y,
        })
    }

    fn is_vertical(&self) -> bool {
        self.start_x == self.end_x
    }

    fn is_horizontal(&self) -> bool {
        self.start_y == self.end_y
    }
}

fn read_input(filename: &str) -> std::io::Result<(Vec<Line>, usize, usize)> {
    println!("Specified file name {}", filename);
    let mut reader = BufReader::new(File::open(filename)?);

    let mut line = String::new();
    let mut len = reader.read_line(&mut line)?;
    let mut line_vector = Vec::<Line>::new();
    let mut dim_x = 0;
    let mut dim_y = 0;

    while len > 0 {
        println!("Line '{}'", line.trim_end());

        let newline = Line::from_string(line.trim_end()).unwrap();

        if newline.start_x > dim_x {
            dim_x = newline.start_x
        }
        if newline.end_x > dim_x {
            dim_x = newline.end_x
        }
        if newline.start_y > dim_y {
            dim_y = newline.start_y
        }
        if newline.end_y > dim_y {
            dim_y = newline.end_y
        }

        line_vector.push(newline);

        line.clear();
        len = reader.read_line(&mut line)?;
    }

    if dim_x > dim_y { dim_y = dim_x; }
    if dim_y > dim_x { dim_x = dim_y; }

    Ok((line_vector, dim_x, dim_y))
}

struct Board {
    max_x : usize,
    max_y : usize,
    data: Vec<Vec<u32>>,
}

impl Board {
    fn init_board(max_x : usize, max_y: usize) -> Board {
        let mut data = Vec::<Vec<u32>>::new();
        for _ in 0..max_y + 1 {
            let mut x = Vec::<u32>::new();
            for _ in 0..max_x + 1 {
                x.push(0);
            }
            data.push(x);
        }
        
        Board { max_x, max_y, data }
    }

    fn apply(& mut self, line: Line) {
        println!("apply( ({},{}) -- ({},{}) )", line.start_x, line.start_y, line.end_x, line.end_y);
        if line.is_horizontal() { 
            if line.end_x > line.start_x {
                let dx = line.end_x - line.start_x;
                for x in 0..dx+1 {
                    self.data[line.start_x + x][line.start_y] += 1;
                }
            } else {
                let dx = line.start_x - line.end_x;
                for x in 0..dx+1 {
                    self.data[line.end_x + x][line.start_y] += 1;
                }
            }
        } else if line.is_vertical() {
            if line.end_y > line.start_y {
                let dy = line.end_y - line.start_y;
                for y in 0..dy+1 {
                    self.data[line.start_x][line.start_y + y] += 1;
                }
            } else {
                let dy = line.start_y - line.end_y;
                for y in 0..dy+1 {
                    self.data[line.start_x][line.end_y + y] += 1;
                }
            }

        } else {
            let diff : usize = (line.end_x as isize - line.start_x as isize).abs() as usize;

            let mut dx: i32 = 1;
            let mut dy: i32 = 1;

            if line.start_x > line.end_x { dx = -1; }
            if line.start_y > line.end_y { dy = -1; }

            for i in 0..diff + 1 {
                let newx = (line.start_x as i32 + (dx * i as i32)).abs() as usize;
                let newy = (line.start_y as i32 + (dy * i as i32)).abs() as usize;
                self.data[newx][newy] += 1;
            }
        }
    }

    fn print(&self) {
        for x in 0..self.max_x + 1 {
            for y in 0..self.max_y + 1 {
                let val = self.data[y][x];
                if val == 0 { 
                    print!(". ");
                } else {
                    print!("{} ", val);
                }
            }
            println!();
        }
    }

    fn count_danger(&self) -> u32 {
        let mut result = 0;
        for x in 0..self.max_x + 1 {
            for y in 0..self.max_y + 1 {
                if self.data[y][x] >= 2 {
                    result += 1;
                }
            }
        }
        result
    }
}

fn run(filename: &str) -> std::io::Result<()> {
    let (line_vector, max_x, max_y) = read_input(filename)?;
    println!(
        "{} lines. Dimensions: {} x {}",
        line_vector.len(),
        max_x,
        max_y
    );

    let mut board = Board::init_board(max_x, max_y);
    println!("Board {}x{}", board.max_x, board.max_y);

    for line in line_vector.into_iter() {
        board.apply(line);
        //board.print();
        //println!();
    }

    println!("Dangerous: {}", board.count_danger());

    Ok(())
}

fn main() -> std::io::Result<()> {
    let matches = App::new("day5")
        .version("1.0.")
        .author("Bjoern Doebel <bjoern.doebel@gmail.com")
        .about("Advent of Code 2021, day 5")
        .arg("<FILE> 'Specify input file'")
        .get_matches();

    if let Some(filename) = matches.value_of("FILE") {
        return run(filename);
    }

    Ok(())
}
