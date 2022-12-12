use std::io::{self, BufRead};

#[derive(Debug)]
enum Instr {
    Addx(i32),
    Noop,
}

struct System {
    rex_x: i32,
    clock: i32,
    strength: i32,
    crt: Vec<char>,
} impl System {
    fn new() -> System {
        System { rex_x: 1, clock: 0, strength: 0, crt: Vec::new() }
    }

    fn exec(&mut self, instr: Instr) {
        match instr {
            Instr::Addx(i) => {self.inc(); self.inc(); self.rex_x += i;},
            Instr::Noop    => self.inc(),
        }
    }

    fn inc(&mut self) {
        if self.rex_x - 1 <= self.clock % 40 && self.clock % 40 <= self.rex_x +1 {
            self.crt.push('#');
        } else {            
            self.crt.push('.');
        }
        self.clock += 1;     
        if self.clock % 40 == 0 {
            self.crt.push('\n');
        }   

        if (self.clock+20) % 40 == 0 {
            self.strength += self.clock * self.rex_x;
            println!("Cycle: {}, X: {}, strength: {}", self.clock, self.rex_x, self.clock * self.rex_x);
        }
    }
}

fn main() {
    let instructions: Vec<Instr> = read_input().map(read_instruction).collect();

    let mut system = System::new();

    for instr in instructions {
        system.exec(instr);
    }
    println!("sum strength: {}", system.strength);

    println!("{}", system.crt.iter().collect::<String>());

}

fn read_input() -> impl Iterator<Item = String> {
    let file = std::fs::File::open("input.txt").expect("could not open input.txt");
    io::BufReader::new(file).lines().filter_map(|line| line.ok())
}

fn read_instruction(line: String) -> Instr {
    let mut words = line.split_whitespace();
    match words.next() {
        Some("noop") => Instr::Noop,
        Some("addx") => Instr::Addx(words.next().unwrap().parse().unwrap()),
        _ => panic!("unknown instruction: {}", line)
    }
}