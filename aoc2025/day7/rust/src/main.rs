use std::{
    io::{BufRead, stdin},
    iter,
};

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

fn p1(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let first = input.next().unwrap();
    let idx = first
        .as_ref()
        .as_bytes()
        .iter()
        .position(|b| *b == b'S')
        .unwrap();
    let mut lanes = Vec::with_capacity(128);
    lanes.push(idx);
    let mut new_lanes = Vec::with_capacity(128);
    let mut total = 0;
    for l in input {
        let l = l.as_ref();
        let bytes = l.as_bytes();
        new_lanes.clear();
        for &i in lanes.iter() {
            if bytes[i] == b'^' {
                total += 1;
                new_lanes.push(i + 1);
                new_lanes.push(i - 1);
            } else {
                new_lanes.push(i);
            }
        }

        new_lanes.sort_unstable();
        new_lanes.dedup();

        lanes.clear();
        lanes.extend_from_slice(&new_lanes);
    }

    total
}
fn p2(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let first = input.next().unwrap();
    let idx = first
        .as_ref()
        .as_bytes()
        .iter()
        .position(|b| *b == b'S')
        .unwrap();
    let mut lanes = iter::repeat_n(0u16, first.as_ref().as_bytes().len()).collect::<Vec<_>>();
    let mut new_lanes = lanes.clone();
    lanes[idx] = 1;
    for l in input {
        let l = l.as_ref();
        let bytes = l.as_bytes();
        for (i, cnt) in lanes.iter().enumerate().filter(|(_, cnt)| **cnt > 0) {
            if bytes[i] == b'^' {
                new_lanes[i + 1] += cnt;
                new_lanes[i - 1] += cnt;
                new_lanes[i] = 0;
            } else {
                new_lanes[i] = *cnt;
            }
        }

        dbg!(&new_lanes);
        lanes.copy_from_slice(&new_lanes);
    }

    lanes.into_iter().map(|v| v as u64).sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_reference() {
        let input = vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];

        assert_eq!(21, p1(input.into_iter()));
    }

    #[test]
    fn p2_reference() {
        let input = vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];

        assert_eq!(40, p2(input.into_iter()));
    }
}
