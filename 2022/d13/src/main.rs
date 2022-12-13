#![feature(iter_array_chunks)]

use std::io::{self, BufRead};
use serde::Deserialize;
use Packet::*;
use std::cmp::Ordering;
use std::cmp::Ordering::*;

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(untagged)]
enum Packet{
    List(Vec<Packet>),
    Int(u32),
} impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Int(i1), Int(i2))   => i1.cmp(i2),
            (List(l1), List(l2)) => {
                for (p1, p2) in l1.iter().zip(l2.iter()) {
                    match p1.cmp(p2) {
                        Less   => return Less,
                        Greater    => return Greater,
                        Equal => {},
                    }
                }
                l1.len().cmp(&l2.len())
            },
            (Int(i1), l2) => List(vec![Int(*i1)]).cmp(l2),
            (l1, Int(i2)) => l1.cmp(&List(vec![Int(*i2)])),
        }
    }
} impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut in_order: Vec<usize> = Vec::new();
    for (idx, [line1, line2, _]) in read_input().array_chunks().enumerate() {
        let p1: Packet = serde_json::from_str(&line1).unwrap();
        let p2: Packet = serde_json::from_str(&line2).unwrap();

        match p1.cmp(&p2) {
            Less | Equal => in_order.push(idx+1),
            Greater     => {}
        }
    }
    println!("Sum indices is {}, with the Packets: {:?}", in_order.iter().sum::<usize>(), in_order);
    
    let divider_packet_1: Packet = serde_json::from_str("[[2]]").unwrap();
    let divider_packet_2: Packet = serde_json::from_str("[[6]]").unwrap();

    let mut packets: Vec<Packet> = read_input().flat_map(|line| serde_json::from_str(&line)).collect();
    packets.push(divider_packet_1.clone());
    packets.push(divider_packet_2.clone());    
    packets.sort();

    let pos1 = packets.binary_search(&divider_packet_1).unwrap() +1;
    let pos2 = packets.binary_search(&divider_packet_2).unwrap() +1;
    println!("Decoder Key is {}*{}={}", pos1, pos2, pos1*pos2);
}

fn read_input() -> impl Iterator<Item = String> {
    let file = std::fs::File::open("input.txt").expect("could not open input.txt");
    io::BufReader::new(file).lines().filter_map(|line| line.ok())
}
