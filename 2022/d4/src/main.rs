use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let overlaps = read_input()
        .filter(|line| pair_overlap(line, false))
        .count();
    println!("complete overlaps: {}", overlaps);

    let overlaps = read_input()
        .filter(|line| pair_overlap(line, true))
        .count();
    println!("partial overlaps: {}", overlaps);
}

fn read_input() -> impl Iterator<Item = String> {
    let file = File::open("input.txt").expect("could not open input.txt");
    io::BufReader::new(file).lines().filter_map(|line| line.ok())
}

fn pair_overlap(elves: &str, partial: bool) -> bool {
    let (elf1, elf2) = elves.split_once(",").expect(&format!("line has no comma: {}", elves));

    let (e1l, e1u) = get_section(elf1);
    let (e2l, e2u) = get_section(elf2);

    if partial {
        !(e1u < e2l || e2u < e1l)
    } else {
        (e1l <= e2l && e1u >= e2u) || (e2l <= e1l && e2u >= e1u)
    }
}

fn get_section(elf: &str) -> (u32, u32) {
    let (lower, upper) = elf.split_once("-").expect(&format!("elf has no -: {}", elf));
    (lower.parse().expect(&format!("not a number: {}", lower)), upper.parse().expect(&format!("not a number {}", upper)))
}
