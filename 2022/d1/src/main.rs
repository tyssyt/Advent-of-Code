use std::fs::File;
use std::io::{self, BufRead, Lines};

fn main() {
    let mut elves = parse_elves("input.txt");
    elves.sort();

    println!("Snackboi: {}", elves.last().unwrap());
    println!("Top 3: {}", elves.iter().rev().take(3).sum::<u32>());
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
