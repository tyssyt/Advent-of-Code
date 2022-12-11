#![feature(iter_next_chunk)]

use std::io::{self, BufRead};
use std::collections::HashSet;

type Pos = (i32, i32);

enum Direction {
    U, R, D, L
} impl Direction {
    fn move_from(&self, pos: Pos) -> Pos {
        match self {
            Direction::U => (pos.0, pos.1+1),
            Direction::R => (pos.0+1, pos.1),
            Direction::D => (pos.0, pos.1-1),
            Direction::L => (pos.0-1, pos.1),
        }
    }
}
impl TryFrom<&str> for Direction {
    type Error = String;
    fn try_from(item: &str) -> Result<Self, Self::Error> {
        match item {
            "U" => Ok(Direction::U),
            "R" => Ok(Direction::R),
            "D" => Ok(Direction::D),
            "L" => Ok(Direction::L),
             _  => Err(item.to_owned()),
        }
    }
}

struct Rope {
    pieces: Vec<Pos>, // positions are relativ to the one behind, position of the last piece is absolute
} impl Rope {
    fn new(pieces: usize) -> Rope {
        Rope { pieces: vec!((0,0); pieces) }
    }

    fn move_head(&mut self, dir: Direction) {
        self.pieces[0] = dir.move_from(self.pieces[0]);
        for idx in 0..self.pieces.len()-1 {
            match self.pieces[idx] {
                ( 2, 2) => self.move_tail(idx, ( 1,1)),
                ( 2,-2) => self.move_tail(idx, ( 1,-1)),
                (-2, 2) => self.move_tail(idx, (-1, 1)),
                (-2,-2) => self.move_tail(idx, (-1,-1)),
                ( 2, y) => self.move_tail(idx, ( 1, y)),
                (-2, y) => self.move_tail(idx, (-1, y)),
                ( x, 2) => self.move_tail(idx, ( x, 1)),
                ( x,-2) => self.move_tail(idx, ( x,-1)),
                _ => ()
            }
        }
    }

    fn move_tail(&mut self, idx: usize, dir: Pos) {
        self.pieces[idx] = (self.pieces[idx].0 - dir.0, self.pieces[idx].1 - dir.1);
        self.pieces[idx+1] = (self.pieces[idx+1].0 + dir.0, self.pieces[idx+1].1 + dir.1);
    }

    fn last(&self) -> Pos {
        *self.pieces.last().unwrap()
    }
}

fn main() {
    move_rope(2);
    move_rope(10);
}

fn move_rope(pieces: usize) {    
    let mut rope = Rope::new(pieces);
    let mut positions = HashSet::new();
    positions.insert(rope.last());

    for line in read_input() {
        let [dir, count] = line.split_whitespace().next_chunk().unwrap();        
        for _ in 0..count.parse::<u32>().expect(&format!("not an integer: {}", count)) {
            rope.move_head(dir.try_into().unwrap());
            positions.insert(rope.last());
        }        
    }
    println!("Tail was at {} unique positions for a rope with {} pieces", positions.len(), pieces);
}

fn read_input() -> impl Iterator<Item = String> {
    let file = std::fs::File::open("input.txt").expect("could not open input.txt");
    io::BufReader::new(file).lines().filter_map(|line| line.ok())
}
