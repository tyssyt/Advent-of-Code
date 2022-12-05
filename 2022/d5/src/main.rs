#![feature(iter_next_chunk)]

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    idk(true);
    idk(false);
}

fn idk(is_crate_mover9000: bool) {
    let mut lines = read_input();

    let mut stacks = parse_stacks(lines.by_ref().take_while(|line| line.starts_with("[")));

    for instruction in lines.skip(1) {
        parse_instruction(&mut stacks, &instruction, is_crate_mover9000).expect(&format!("failed to parse Instruction: {}", &instruction))
    }    
    println!("Top of Stacks: {:?}", stacks.iter().filter_map(|stack| stack.last()).collect::<String>());
}

fn read_input() -> impl Iterator<Item = String> {
    let file = File::open("input.txt").expect("could not open input.txt");
    io::BufReader::new(file).lines().filter_map(|line| line.ok())
}

fn parse_stacks(lines: impl Iterator<Item = String>) -> Vec<Vec<char>> {
    let mut peek = lines.peekable();

    let number_of_stacks = 1 + (peek.peek().unwrap().chars().count() / 4);
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); number_of_stacks];

    // build the stacks
    for line in peek {
        line.chars()
            .skip(1).step_by(4)
            .zip(stacks.iter_mut())
            .filter(|(c, _)| c != &' ')
            .for_each(|(c, stack)| stack.push(c));
    }

    // inverse stacks
    for stack in &mut stacks {
        stack.reverse();
    }

    stacks
}

fn parse_instruction(stacks: &mut Vec<Vec<char>>, instruction: &str, is_crate_mover9000: bool) -> Option<()> {       
    let [amount, from, to] = instruction
        .split_whitespace()
        .skip(1).step_by(2)
        .filter_map(|word| word.parse::<usize>().ok())
        .next_chunk()
        .ok()?;

    if is_crate_mover9000 {
        for _ in 0..amount {
            let moved_crate = stacks.get_mut(from-1)?.pop()?;
            stacks.get_mut(to-1)?.push(moved_crate);
        }
    } else {
        let from_stack = stacks.get_mut(from-1)?;
        let moved_crates = from_stack.split_off(from_stack.len() - amount);
        stacks.get_mut(to-1)?.extend(moved_crates);
    }

    Some(())
}