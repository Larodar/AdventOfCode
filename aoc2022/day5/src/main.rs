use std::{
    fmt::Display,
    io::{stdin, Read},
    num::ParseIntError,
    str,
    str::FromStr,
};

fn main() {
    match std::env::args()
        .into_iter()
        .skip(1)
        .next()
        .map(|s| s.parse::<u32>().unwrap())
    {
        Some(1) => println!("{}", p1(stdin().bytes().map(|b| b.unwrap()))),
        Some(2) => println!("{}", p2(stdin().bytes().map(|b| b.unwrap()))),
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}

struct Stack(Vec<u8>);

impl Stack {
    fn push(&mut self, item: u8) {
        self.0.push(item)
    }

    fn pop(&mut self) -> Option<u8> {
        self.0.pop()
    }
}

struct Cargo {
    stacks: Vec<Stack>,
}

impl Cargo {}

fn p1<I: Iterator<Item = u8>>(iter: I) -> u64 {
    0
}

fn p2<I: Iterator<Item = u8>>(iter: I) -> u64 {
    0
}
