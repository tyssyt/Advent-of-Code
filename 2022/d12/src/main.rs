use std::fs;
use std::ops::{Index, IndexMut};
use itertools::Itertools;

type Coord = (usize, usize);

#[derive(Debug)]
struct Grid <T> {
    width: usize,
    height: usize,
    elements: Vec<T>,
} impl<T> Grid<T> {
    fn new(width: usize, elements: Vec<T>) -> Grid<T> {
        let height = elements.len() / width;
        assert!(width*height == elements.len());
        Grid{width, height, elements}
    }

    fn idx_to_coord(&self, idx: usize) -> Coord {
        (idx % self.width, idx / self.width)
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

impl Grid<u32> {
    fn predecessors(&self, from: Coord) -> Vec<(Coord, u32)> {
        let mut successors = Vec::new();
        let reachable_height = self[from] -1;

        if from.0 <  self.width-1 && self[(from.0+1, from.1)] >= reachable_height {successors.push(((from.0+1, from.1), 2))}
        if from.0 >             0 && self[(from.0-1, from.1)] >= reachable_height {successors.push(((from.0-1, from.1), 2))}
        if from.1 < self.height-1 && self[(from.0, from.1+1)] >= reachable_height {successors.push(((from.0, from.1+1), 2))}
        if from.1 >             0 && self[(from.0, from.1-1)] >= reachable_height {successors.push(((from.0, from.1-1), 2))}
        successors
    }

    fn distance(&self, from: Coord, to: Coord) -> u32 {
          ((from.0 as i32 - to.0 as i32).abs() as u32)
        + ((from.1 as i32 - to.1 as i32).abs() as u32)
        +  self[from] - self[to]
    }
}

fn main() {
    let (start, end, grid) = parse_grid();

    if let Some((solution, _)) = pathfinding::prelude::astar(&end, |p| grid.predecessors(*p), |p| grid.distance(*p, start), |p| *p==start) {
        println!("{} Steps are required to get the best signal!", solution.len()-1);
    } else {
        println!("could not find solution");
    }

    let starts: Vec<Coord> = grid.coords().filter(|c| grid[*c] == 1).collect();
    if let Some((solution, _)) = pathfinding::prelude::astar(&end, |p| grid.predecessors(*p), |p| grid.distance(*p, start), |p| starts.contains(p)) {
        println!("The best hiking path starts at {:?} and reqiures only {} steps!", solution.last().unwrap(), solution.len()-1);
    } else {
        println!("could not find solution");
    }

}

fn parse_grid() -> (Coord, Coord, Grid<u32>) {
    let file = fs::read_to_string("input.txt").expect("could not read input.txt");
    let width = file.find("\n").unwrap();

    let mut start_idx: usize = usize::MAX; 
    let mut end_idx: usize = usize::MAX; 

    let trees = file.chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .map(|(i,c)| match c {
            'S' => {start_idx = i; 1},
            'E' => {end_idx = i; 26},
             _  => c as u32 - 'a' as u32 +1
        })
        .collect();

    let grid = Grid::new(width, trees);
    (grid.idx_to_coord(start_idx), grid.idx_to_coord(end_idx), grid) 
}
