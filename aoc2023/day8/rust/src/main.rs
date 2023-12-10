use std::io::{stdin, BufRead};

fn main() {
    match std::env::args()
        .nth(1)
        .map(|s| s.parse::<u32>().unwrap())
    {
            Some(1) => println!("{}", p1(stdin().lock().lines().map(|line_result| line_result.unwrap()))),
        Some(2) => println!("{}", p2(stdin().lock().lines().map(|line_result| line_result.unwrap()))),
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}

fn p1(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {0}
fn p2(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {0}
