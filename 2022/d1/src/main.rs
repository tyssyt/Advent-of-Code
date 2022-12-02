use std::env;
use std::fs::File;
use std::io::{self, BufRead, Lines};

fn main() {
    let file: String = env::args().nth(1).expect("no input file specified");

    let mut elves = parse_elves(& file);
    elves.sort();
    elves.reverse();

    println!("Snackboi: {}", elves[0]);
    println!("Top 3: {}", elves[0] + elves[1] + elves[2]);
}

fn parse_elves(filename: &str) -> Vec<u32> {    
    let mut lines = read_lines(filename).expect(&format!("could not open file {}", filename));
    let mut elves: Vec<u32> = Vec::new();

    while let Some(calories) = parse_elf(&mut lines) {
        elves.push(calories);
    }
    return elves;
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_elf(lines: &mut Lines<io::BufReader<File>>) -> Option<u32> {
    let calories = lines
        .filter_map(|line| line.ok())
        .take_while(|line| !line.is_empty())
        .filter_map(|line| line.parse::<u32>().ok())
        .sum();
    if calories != 0 {Some(calories)} else {None}
}
