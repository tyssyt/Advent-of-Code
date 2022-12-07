use std::fs;
use std::borrow::Cow;

fn main() {
    let input = fs::read_to_string("input.txt").expect("could not read input.txt");
    {
        let (idx, marker) = find_marker(&input, 4);
        println!("{} found at {}", marker, idx);
    }
    {
        let (idx, marker) = find_marker(&input, 14);
        println!("{} found at {}", marker, idx);
    }

}

fn find_marker(input: &str, size: usize) -> (usize, Cow<str>) {
    input
        .as_bytes()
        .windows(size)
        .enumerate()
        .find(|(_, chars)| all_distinct(chars))
        .map(|(idx, chars)| (idx+size, String::from_utf8_lossy(chars)))
        .expect("could not find marker")
}

fn all_distinct(chars: &[u8]) -> bool {
    for (idx, c) in chars.iter().enumerate() {
        for o in &chars[idx+1..] {
            if c == o {
                return false
            }
        }
    };
    true
}