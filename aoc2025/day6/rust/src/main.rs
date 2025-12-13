use std::io::{BufRead, stdin};

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
    let mut lines = vec![];
    let mut ops = vec![];
    for l in input {
        let l = l.as_ref();
        let first_char = l.trim_start().as_bytes()[0];
        if matches!(first_char, b'*' | b'+') {
            l.split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| {
                    if s.as_bytes()[0] == b'+' {
                        Op::Add
                    } else {
                        Op::Mul
                    }
                })
                .for_each(|o| ops.push(o));
        } else {
            lines.push(
                l.split_ascii_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
    }

    let mut total = 0;
    for n in 0..ops.len() {
        // fetch the nth item from each line and fold them using the op
        total += if ops[n] == Op::Add {
            lines.iter().map(|l| l[n]).fold(0, |acc, val| acc + val)
        } else {
            lines.iter().map(|l| l[n]).fold(1, |acc, val| acc * val)
        }
    }

    total
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Op {
    Skip,
    Add,
    Mul,
}

fn p2(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let mut lines = vec![];
    let mut ops = Vec::with_capacity(1024);
    for l in input {
        let l = l.as_ref();
        let first_char = l.trim_start().as_bytes()[0];
        if matches!(first_char, b'*' | b'+') {
            let bytes = l.as_bytes();
            let mut state = if bytes[0] == b'+' { Op::Add } else { Op::Mul };
            ops.push(state);
            for i in 1..bytes.len() {
                let b = bytes[i];
                if b == b'+' {
                    state = Op::Add;
                    ops[i - 1] = Op::Skip;
                } else if b == b'*' {
                    state = Op::Mul;
                    ops[i - 1] = Op::Skip;
                }

                ops.push(state);
            }
        } else {
            lines.push(
                l.as_bytes()
                    .iter()
                    .map(|b| if *b == b' ' { 0xFFu8 } else { b - b'0' })
                    .collect::<Vec<_>>(),
            );
        }
    }

    let ten_pow = (0..lines.len())
        .rev()
        .map(|i| 10u64.pow(i as u32))
        .collect::<Vec<_>>();

    dbg!(&lines);
    dbg!(&ops);
    let mut total = 0;
    let mut buf = [0u8; 32];
    let mut last_op = Op::Skip;
    let mut acc = 0;
    for n in 0..ops.len() {
        if last_op == Op::Skip {
            acc = if ops[n] == Op::Mul { 1 } else { 0 };
        }

        let mut idx = 0;
        for b in lines.iter().map(|l| l[n]).filter(|v| *v != 0xFF) {
            buf[idx] = b;
            idx += 1;
        }

        let slice = &buf[..idx];
        let pow_slice = &ten_pow[ten_pow.len() - idx..ten_pow.len()];

        let val = slice
            .iter()
            .zip(pow_slice.iter())
            .map(|(&v, &exp)| v as u64 * exp)
            .fold(0, |acc, val| acc + val);
        if ops[n] == Op::Add {
            acc += val;
        } else if ops[n] == Op::Mul {
            acc *= val;
        } else {
            total += acc;
        };

        last_op = ops[n];
    }

    total + acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_reference() {
        let input = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];

        assert_eq!(4277556, p1(input.iter()));
    }

    #[test]
    fn p2_reference() {
        let input = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];

        assert_eq!(3263827, p2(input.iter()));
    }

    #[test]
    fn p2_test_1() {
        let input = vec!["1    ", "  1  ", "    1", "* * *"];

        assert_eq!(3, p2(input.iter()));
    }

    #[test]
    fn p2_test_2() {
        let input = vec!["1    ", "  1  ", "    1", "+ + +"];

        assert_eq!(3, p2(input.iter()));
    }
}
