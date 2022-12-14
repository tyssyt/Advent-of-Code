use std::io::{self, BufRead};
use std::ops::{Index, IndexMut};
use itertools::Itertools;
use std::cmp::{min, max};
use CaveFilling::*;

type Coord = (usize, usize);

#[derive(Debug, Clone, PartialEq)]
enum CaveFilling {
    Air, Sand, Rock,
} impl CaveFilling {
    fn draw(&self) -> char {
        match self {
            Air  => '.',
            Sand => 'o',
            Rock => '#',
        }
    }
}

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    height: usize,
    elements: Vec<T>,
} impl<T: Clone> Grid<T> {
    fn new(width: usize, height: usize, fill: T) -> Grid<T> {
        Grid{width, height, elements: vec![fill; width*height]}
    } 

    fn coords(&self) -> impl Iterator<Item = Coord> {
        (0..self.width).cartesian_product(0..self.height)
    }
}
impl<T> Index<Coord> for Grid<T> {
    type Output = T;
    fn index(&self, index: Coord) -> &Self::Output {
        &self.elements[index.0 + index.1 * self.width]
    }
}
impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.elements[index.0 + index.1 * self.width]
    }
}

fn main() {
    run(false);
    run(true);
}

fn run(include_floor: bool) {
    let mut cave = build_cave(include_floor);

    let mut sand_counter = 0;
    while let Some(sand) = flow_sand((500, 0), &cave) {
        cave[sand] = Sand;
        sand_counter += 1;

        if sand == (500, 0) {
            break;
        }
    } 

    println!("Finished:");
    print_cave(&cave);
    println!("The cave is filled with {} units of sand", sand_counter);
}

fn build_cave(include_floor: bool) -> Grid<CaveFilling> {
    let rocks: Vec<Vec<Coord>> = read_input().map(read_rock).collect();
    let height = rocks.iter().flat_map(|r| r.iter()).map(|(_, y)| *y).max().unwrap();

    let mut cave = Grid::new(1000, height+3, Air);
    for rock in rocks {
        for (from, to) in rock.iter().tuple_windows() {
            for x in min(from.0, to.0)..max(from.0, to.0)+1 {
                for y in min(from.1, to.1)..max(from.1, to.1)+1 {
                    cave[(x,y)] = Rock;
                }
            }
        }
    }

    if include_floor {
        for x in 0..cave.width {
            cave[(x,height+2)] = Rock;
        }
    }
    cave
}

fn read_rock(line: String) -> Vec<Coord> {
    line
        .split(" -> ")
        .filter_map(|c| c.split_once(","))
        .map(|(i1, i2)| (i1.parse().unwrap(), i2.parse().unwrap()))
        .collect()
}

fn flow_sand(origin: Coord, cave: &Grid<CaveFilling>) -> Option<Coord> {
    let mut cur = origin;
    while cur.1 < cave.height-1 {
        if cave[(cur.0, cur.1+1)] == Air {
            cur = (cur.0, cur.1+1);
        } else if cave[(cur.0-1, cur.1+1)] == Air {
            cur = (cur.0-1, cur.1+1);
        } else if cave[(cur.0+1, cur.1+1)] == Air {
            cur = (cur.0+1, cur.1+1);
        } else {
            return Some(cur);
        }
    }
    None
}

fn read_input() -> impl Iterator<Item = String> {
    let file = std::fs::File::open("input.txt").expect("could not open input.txt");
    io::BufReader::new(file).lines().filter_map(|line| line.ok())
}

fn print_cave(cave: &Grid<CaveFilling>) {
    let min_x = cave.coords()
        .filter(|c| c.1 < cave.height-1 && cave[*c] != Air)
        .map(|(x, _)| x)
        .min()
        .unwrap();
    let max_x = cave.coords()
        .filter(|c|  c.1 < cave.height-1 && cave[*c] != Air)
        .map(|(x, _)| x)
        .max()
        .unwrap();


    for y in 0..cave.height {
        for x in min_x..max_x+1 {
            print!("{}", cave[(x,y)].draw());
        }
        println!("");
    }
}