#![feature(iter_next_chunk)]

use std::iter::Peekable;
use std::ptr;
use std::io::{self, BufRead};

type File = (String, u32);

struct Dir {
    name: String,    
    children: Vec<Dir>,
    files: Vec<File>,
    parent: *mut Dir,
} impl Dir {

    fn new(name: String, parent: Option<*mut Dir>) -> Dir {
        Dir{name, children: Vec::new(), files: Vec::new(), parent: parent.unwrap_or(ptr::null_mut())}
    }

    fn push_child(&mut self, name: String) {
        let self_raw: *mut Dir = self;
        self.children.push(Dir::new(name.to_owned(), Some(self_raw)))
    }

    fn compute_sizes<'a>(&'a self, sizes: &mut Vec<(&'a Dir, u32)>) -> u32 {
        let mut size = 0;
        for file in &self.files {
            size += file.1;
        }
        for child in &self.children {
            size += child.compute_sizes(sizes);
        }

        sizes.push((self, size));
        size
    }
}

struct FileSystem {
    root: Dir,
    current: *mut Dir,
} impl FileSystem {

    fn new() -> FileSystem {
        let mut fs = FileSystem { root: Dir::new("/".to_string(), None), current: ptr::null_mut()};
        fs.change_to_root();
        fs
    }

    fn get_current_mut(&mut self) -> &mut Dir {
        unsafe {&mut *self.current}
    }

    fn change_to_root(&mut self) {
        self.current = &mut self.root;
    }

    fn change_up(&mut self) {
        if let Some(parent) = unsafe {(*self.current).parent.as_mut()} {
            self.current = parent;
        }
    }

    fn change_down(&mut self, dir: &str) {
        if let Some(child) = unsafe {(*self.current).children.iter_mut().find(|c| c.name == dir)} {
            self.current = &mut *child;
        }
    }

    fn compute_sizes(&self) -> Vec<(&Dir, u32)> {
        let mut sizes = Vec::new();
        self.root.compute_sizes(&mut sizes);
        sizes
    }
}

fn main() {
    let fs = parse_filesystem();
    let sizes = fs.compute_sizes();

    let star1: u32 = sizes.iter()
        .filter(|(_, size)| *size <= 100000)
        .map(|(_, size)| *size)
        .sum();
    println!("Sum of sizes of dirs > 100000 is {}", star1);

    let root_size = sizes.last().unwrap().1;
    let required_space = root_size - 40000000;
    let (dir, size) = sizes.iter()
        .filter(|(_, size)| *size >= required_space)
        .min_by_key(|(_, size)| *size).unwrap();
    println!("Best dir to delete is {} with size {}", dir.name, size);

}

fn read_input() -> impl Iterator<Item = String> {
    let file = std::fs::File::open("input.txt").expect("could not open input.txt");
    io::BufReader::new(file).lines().filter_map(|line| line.ok())
}

fn parse_filesystem() -> FileSystem {
    let mut fs = FileSystem::new();

    let mut input = read_input().peekable();
    while let Some(line) = input.next() {

        assert!(line.starts_with("$ "));
        let mut words = line.split_whitespace();
        match words.by_ref().skip(1).next() {
            Some("cd") => parse_cd(words.next().expect("cd without argument"), &mut fs),
            Some("ls") => parse_ls(fs.get_current_mut(), input.by_ref()),
            _ => panic!("unknown command {}", line),
        };
    }

    fs
}

fn parse_cd(arg: &str, fs: &mut FileSystem) {
    match arg {
        ".."   => fs.change_up(),
        "/"    => fs.change_to_root(),
        folder => fs.change_down(folder),
    };
}

fn parse_ls(cur: &mut Dir, lines: &mut Peekable<impl Iterator<Item = String>>) {
    while let Some(line) = lines.next_if(|line| !line.starts_with("$")) {
        match line.split_whitespace().next_chunk().unwrap() {
            ["dir", name] => cur.push_child(name.to_owned()),
            [size, name] => cur.files.push((name.to_owned(), size.parse().expect(&format!("expected int, found: {}", size)))),
        }
    }
}
