use std::fs::File;
use std::io::{self, BufRead};

#[derive(PartialEq, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}
impl RPS {
    fn value(&self) -> u32 {
        match *self {
            RPS::Rock     => 1,
            RPS::Paper    => 2,
            RPS::Scissors => 3,
        }
    }
    fn next(&self) -> RPS {
        match *self {
            RPS::Rock     => RPS::Scissors,
            RPS::Paper    => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
    fn result(&self, opponent: &RPS) -> WLD {
        if      self == opponent         { return WLD::Draw;}
        else if self == &opponent.next() { return WLD::Loose; }
        else                             { return WLD::Win; }
    }
    fn find_move_that(self, result: &WLD) -> RPS {
        match result {
            WLD::Win   => self.next().next(),
            WLD::Draw  => self,
            WLD::Loose => self.next(),             
        }
    }
}
impl TryFrom<&str> for RPS {
    type Error = ();

    fn try_from(symbol: &str) -> Result<Self, Self::Error> {
        match symbol {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug)]
enum WLD {
    Win,
    Loose,
    Draw,
}
impl WLD {
    fn value(&self) -> u32 {
        match *self {
            WLD::Win   => 6,
            WLD::Loose => 0,
            WLD::Draw  => 3,
        }
    }
}
impl From<RPS> for WLD {
    fn from(rps: RPS) -> Self {
        match rps {
            RPS::Rock     => WLD::Loose,
            RPS::Paper    => WLD::Draw,
            RPS::Scissors => WLD::Win,
        }
    }
}


fn main() {
    let score1: u32 = read_input()
    .map(parse_line)
    .map(|(opp, me)| me.value() + me.result(&opp).value())
    .sum();    
    println!("Score1: {}", score1);

    let score2: u32 = read_input()
    .map(parse_line)
    .map(|(opp, tactic)| {let wld: WLD = tactic.into(); opp.find_move_that(&wld).value() + wld.value() })
    .sum();
    println!("Score2: {}", score2);
}

fn read_input() -> impl Iterator<Item = String> {
    let file = File::open("input.txt").expect("could not open input.txt");
    io::BufReader::new(file).lines().filter_map(|line| line.ok())
}

fn parse_line(line: String) -> (RPS, RPS) {
    let mut symbols = line.split_whitespace().filter_map(|s| s.try_into().ok());
    (symbols.next().unwrap(), symbols.next().unwrap())
}

