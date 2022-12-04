#![feature(iter_array_chunks)]

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let sum_priorities: u32 = read_input().map(find_duplicate).map(get_prio).sum();
    println!("Sum of Errors: {}", sum_priorities);

    let sum_badge: u32 = read_input()
        .map(count_items)
        .array_chunks()
        .map(|[elf1, elf2, elf3]| find_badge(elf1, elf2, elf3))
        .sum();

    println!("Sum of Badges: {}", sum_badge);

}

fn read_input() -> impl Iterator<Item = String> {
    let file = File::open("input.txt").expect("could not open input.txt");
    io::BufReader::new(file).lines().filter_map(|line| line.ok())
}

fn find_duplicate(line: String) -> char {
    let mut items = line.chars();
    let compartment1: Vec<char> = items.by_ref().take(line.len()/2).collect();

    items.find(|i| compartment1.contains(i))        
        .expect(&format!("no duplicate founds in rucksack {}", line))
}

fn get_prio(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 +1,
        'A'..='Z' => item as u32 - 'A' as u32 +27,        
        _ => panic!("unknown item {}", item)
    }
}

fn count_items(elf: String) -> [u32; 53] {
    let mut counts: [u32; 53] = [0; 53];
    for item in elf.chars() {
        let prio = get_prio(item) as usize;
        counts[prio] += 1;
    }
    counts
}

fn find_badge(elf1: [u32; 53], elf2: [u32; 53], elf3: [u32; 53]) -> u32 {
    for i in 1..54 {
        if elf1[i] > 0 && elf2[i] > 0 && elf3[i] > 0 {
            return i as u32;
        }
    }
    panic!("no common item in Elves: {:?}, {:?} & {:?}", elf1, elf2, elf3)
}