#![feature(iter_next_chunk)]

use std::io::{self, BufRead};
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::BTreeSet;

type Coord = (i32, i32);

#[derive(Debug)]
struct Sensor {
    pos: Coord,
    beacon: Coord,
    distance: i32,
} impl Sensor {
    fn new(pos: Coord, beacon: Coord) -> Self {
        let distance = dist(&pos, &beacon);
        Self { pos, beacon, distance }
    }

    fn intersect(&self, y: i32) -> Option<(i32, i32)> {
        let dist = (y - self.pos.1).abs();

        let overlap = self.distance - dist;
        if overlap < 0 {
            return None
        }

        Some((self.pos.0 - overlap, self.pos.0 + overlap))
    }

    fn edge<'a>(&'a self) -> impl Iterator<Item=Coord> + 'a {
        (1..self.distance+2)
            .map(|d| (self.distance+1 - d, d))
            .flat_map(|(x,y)| [(x,y), (y,-x), (-x,-y), (-y,x)])
            .map(|(x,y)| (x + self.pos.0, y + self.pos.1) )
    }
}

fn dist(c1: &Coord, c2: &Coord) -> i32 {
    (c1.0 - c2.0).abs() + (c1.1 - c2.1).abs()
}

fn main() {
    let sensors: Vec<Sensor> = read_input().filter_map(parse_line).collect();

    let line = 2000000;
    let positions: BTreeSet<i32> = sensors.iter()
        .filter_map(|s| s.intersect(line))
        .flat_map(|(start, end)| start..end+1)
        .collect();
    let other_stuff_in_the_line: BTreeSet<i32> = sensors.iter()
        .flat_map(|s| [s.pos, s.beacon])
        .filter(|(_, y)| *y == line )
        .map(|(x, _)| x)
        .collect();
    println!("{} Positions intersect y={}", positions.len()-other_stuff_in_the_line.len(), line);


    let size = 4000000;
    let mut edge_points = HashMap::new();
    for edge_point in sensors.iter()
        .flat_map(|s| s.edge())
        .filter(|(x,y)| *x >= 0 && *x <= size && *y >= 0 && *y <= size) {
            
        edge_points.entry(edge_point).and_modify(|c| *c += 1).or_insert(1);
    }

    let distress_signal_candidates: Vec<_> = edge_points.into_iter().filter(|(_, c)| *c >= 4).map(|(p, _)| p).collect();
    println!("Distress Signal Candidates are {:?}", distress_signal_candidates);

    'signal: for distress_signal in distress_signal_candidates {
        for sensor in &sensors {
            if dist(&distress_signal, &sensor.pos) <= sensor.distance {
                continue 'signal;
            }
        }
        println!("Distress Signal is at {:?} with Tuning Frequency {}", distress_signal, (distress_signal.0 as i64) * 4000000 + distress_signal.1 as i64);
    }

}

fn read_input() -> impl Iterator<Item = String> {
    let file = std::fs::File::open("input.txt").expect("could not open input.txt");
    io::BufReader::new(file).lines().filter_map(|line| line.ok())
}

fn parse_line(line: String) -> Option<Sensor> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
    }

    let [x1, y1, x2, y2] = RE.captures(&line)?.iter().skip(1)
        .filter_map(|c| c)
        .flat_map(|c| c.as_str().parse::<i32>())
        .next_chunk().unwrap();
    Some( Sensor::new((x1, y1), (x2, y2)) )
}