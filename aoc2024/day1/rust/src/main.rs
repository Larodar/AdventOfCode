use std::io::{stdin, BufRead};

fn main() {
    match std::env::args().nth(1).map(|s| s.parse::<u32>().unwrap()) {
        Some(1) => println!(
            "{}",
            p1(stdin()
                .lock()
                .lines()
                .map(|line_result| line_result.unwrap()))
        ),
        Some(2) => println!(
            "{}",
            p2(stdin()
                .lock()
                .lines()
                .map(|line_result| line_result.unwrap()))
        ),
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}
fn p1(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let mut lhs: Vec<i32> = vec![];
    let mut rhs: Vec<i32> = vec![];
    for line in input {
        let parts = line
            .as_ref()
            .split_once(' ')
            .expect("Unexpected line format.");
        lhs.push(parts.0.parse().expect("Not an integer."));
        rhs.push(parts.1.trim_start().parse().expect("Not an integer."));
    }

    lhs.sort();
    rhs.sort();

    lhs.into_iter()
        .zip(rhs)
        .map(|(l, r)| (l - r).unsigned_abs() as u64)
        .sum()
}

fn p2(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let mut lhs: Vec<u32> = vec![];
    let mut rhs: Vec<u32> = vec![];
    for line in input {
        let parts = line
            .as_ref()
            .split_once(' ')
            .expect("Unexpected line format.");
        lhs.push(parts.0.parse().expect("Not an integer."));
        rhs.push(parts.1.trim_start().parse().expect("Not an integer."));
    }

    lhs.into_iter()
        .map(|item| rhs.iter().filter(|i| **i == item).count() as u64 * item as u64)
        .sum::<u64>()
}
