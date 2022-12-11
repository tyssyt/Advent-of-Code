use std::fs;
use std::ops::{Index, IndexMut};
use itertools::Itertools;

fn main() {
    let trees = parse_grid();

    let star1 = trees.check_visibilities().elements.iter()
        .filter(|v| **v)
        .count();
    println!("visible Trees: {}", star1 +4); // we skipped the corners, add them here

    let star2 = trees.coords()
        .map(|coord| trees.scenic_score(coord))
        .max();
    println!("Scenic Score: {:?}", star2);
}

fn parse_grid() -> Grid<u32> {
    let file = fs::read_to_string("input.txt").expect("could not read input.txt");
    let width = file.find("\n").unwrap();
    let trees = file.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    Grid::new(width, trees)
}

type Coord = (usize, usize);

#[derive(Debug, Clone)]
enum Direction {
    N, E, S, W,
}

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

    fn line(&self, from: Coord, dir: Direction) -> Box<dyn Iterator<Item = Coord>> {
        match dir {
            Direction::N => Box::new((0..from.1+1).rev()   .map(move |y| (from.0, y))),
            Direction::E => Box::new((from.0..self.width)  .map(move |x| (x, from.1))),
            Direction::S => Box::new((from.1..self.height) .map(move |y| (from.0, y))),
            Direction::W => Box::new((0..from.0+1).rev()   .map(move |x| (x, from.1))),
        }
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

    fn check_visibilities(&self) -> Grid<bool> {
        let mut visibilities = Grid::new(self.width, vec![false; self.elements.len()]);
    
        let max_x = self.width  -1;
        let max_y = self.height -1;
    
        for x in 1..max_x {
            self.check_line(&mut visibilities, (x, 0), Direction::S);
            self.check_line(&mut visibilities, (x, max_y), Direction::N);
        }
        for y in 1..max_y {
            self.check_line(&mut visibilities, (0, y), Direction::E);
            self.check_line(&mut visibilities, (max_x, y), Direction::W);
        }
    
        visibilities
    }
    
    fn check_line(&self, visibilities: &mut Grid<bool>, start: Coord, dir: Direction) {
        let mut height: u32 = 0;

        for tree in self.line(start, dir) {
            if self[tree] >= height {
                height = self[tree] + 1;
                visibilities[tree] = true;
                if height == 10 {
                    break;
                }
            }
        }
    }

    fn scenic_score(&self, tree: Coord) -> u32 {
        self.viewing_distance(tree, Direction::N) * self.viewing_distance(tree, Direction::E) * self.viewing_distance(tree, Direction::S) * self.viewing_distance(tree, Direction::W)
    }

    fn viewing_distance(&self, from: Coord, dir: Direction) -> u32 {
        let height_from = self[from];
        let mut larger_tree_found = false;

        let distance = self.line(from, dir)
            .skip(1)
            .map(|tree| self[tree])
            .inspect(|height_other| if height_from <= *height_other {larger_tree_found = true})
            .take_while(|height_other| height_from > *height_other)
            .count() as u32;
        
        if larger_tree_found {distance+1} else {distance}
    }
}
