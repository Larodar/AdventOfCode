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

pub fn p1(input: impl Iterator<Item = impl AsRef<str>>) -> i64 {
    let mut total = 0;
    let mut maps = vec![];
    for line in input {
        maps.clear();
        let l = line
            .as_ref()
            .split(' ')
            .map(|num_str| num_str.parse().unwrap())
            .collect::<Vec<i32>>();
        maps.push(l);
        let mut idx = 0;
        while maps[maps.len() - 1].iter().any(|num| *num != 0) {
            let mut map = Vec::with_capacity(maps[0].len());
            for w in maps[idx][..].windows(2) {
                assert_eq!(w.len(), 2);
                map.push(w[1] - w[0]);
            }

            idx += 1;
            maps.push(map);
        }

        let mut carry = maps[maps.len() - 1][0];
        for m in maps[0..maps.len() - 1].iter().rev() {
            carry = carry + m.last().unwrap();
        }

        total += carry;
    }

    total as i64
}

pub fn p2(input: impl Iterator<Item = impl AsRef<str>>) -> i64 {
    let mut total = 0;
    let mut maps = vec![];
    for line in input {
        maps.clear();
        let l = line
            .as_ref()
            .split(' ')
            .map(|num_str| num_str.parse().unwrap())
            .collect::<Vec<i32>>();
        maps.push(l);
        let mut idx = 0;
        while maps[maps.len() - 1].iter().any(|num| *num != 0) {
            let mut map = Vec::with_capacity(maps[0].len());
            for w in maps[idx][..].windows(2) {
                assert_eq!(w.len(), 2);
                map.push(w[1] - w[0]);
            }

            idx += 1;
            maps.push(map);
        }

        let mut carry = maps[maps.len() - 1][0];
        for m in maps[0..maps.len() - 1].iter().rev() {
            carry = m[0] - carry;
        }

        total += carry;
    }

    total as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = vec!["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];
        assert_eq!(114, p1(input.into_iter()));
    }
}
