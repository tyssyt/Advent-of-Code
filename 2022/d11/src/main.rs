struct Monkey {
    inspect: fn(u64) -> u64,
    test_mod: u64, 
    next_true: usize,
    next_false: usize,
} impl Monkey {
    fn take_turn(&self, items: &Vec<u64>, relief: bool, lcm: u64) -> (Vec<u64>, Vec<u64>) {
        let mut items_true = Vec::new();
        let mut items_false = Vec::new();

        for item in items {
            let mut new_item = (self.inspect)(*item) % lcm;
            if relief { new_item = new_item / 3; }

            if new_item % self.test_mod == 0 {
                items_true.push(new_item);
            } else {
                items_false.push(new_item);
            }
        }

        (items_true, items_false)
    }
}

fn main() {
    let monkeys = vec![
        Monkey{ inspect: |item| item*7,    test_mod: 11, next_true: 5, next_false: 6 },
        Monkey{ inspect: |item| item*17,   test_mod: 19, next_true: 4, next_false: 2 },
        Monkey{ inspect: |item| item+2,    test_mod:  5, next_true: 7, next_false: 4 },
        Monkey{ inspect: |item| item+1,    test_mod:  2, next_true: 2, next_false: 1 },
        Monkey{ inspect: |item| item+6,    test_mod: 13, next_true: 7, next_false: 0 },
        Monkey{ inspect: |item| item*item, test_mod:  7, next_true: 6, next_false: 3 },
        Monkey{ inspect: |item| item+3,    test_mod:  3, next_true: 1, next_false: 3 },
        Monkey{ inspect: |item| item+4,    test_mod: 17, next_true: 0, next_false: 5 },
    ];

    println!("smoll Monkey Business: {}", monkey_business(&monkeys,    20,  true));
    println!("BIG Monkey Business: {}",   monkey_business(&monkeys, 10000, false));
}

fn monkey_business(monkeys: &Vec<Monkey>, rounds: usize, relief: bool) -> usize {
    let mut inspections = vec![0; monkeys.len()];
    let mut items: Vec<Vec<u64>> = vec![
        vec![97, 81, 57, 57, 91, 61],
        vec![88, 62, 68, 90],
        vec![74, 87],
        vec![53, 81, 60, 87, 90, 99, 75],
        vec![57],
        vec![54, 84, 91, 55, 59, 72, 75, 70],
        vec![95, 79, 79, 68, 78],
        vec![61, 97, 67]
    ];
    let lcm = monkeys.iter().map(|m| m.test_mod).reduce(num_integer::lcm).unwrap();

    for round in 0..rounds {
        for idx in 0..monkeys.len() {
            inspections[idx] += items[idx].len();
            let monkey = &monkeys[idx];
            let (mut items_true, mut items_false) = monkey.take_turn(&items[idx], relief, lcm);
            items[idx].clear();
            items[monkey.next_true].append(&mut items_true);
            items[monkey.next_false].append(&mut items_false);
        }
        
        if (round+1) % 1000 == 0 {println!("finished round {}", round+1)};
    }

    inspections.sort();
    inspections.iter().rev().take(2).product()
}
