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
    let mut total = 0;
    for line_ref in input {
        let line = line_ref.as_ref();
        let line_raw = line.as_bytes();

        let mut idx = 0;
        let mut high = line_raw[0];
        for i in 1..line_raw.len() - 1 {
            if line_raw[i] > high {
                high = line_raw[i];
                idx = i;
            }
        }

        let slice = &line_raw[idx + 1..];
        let mut high2 = slice[0];
        for i in 1..slice.len() {
            if slice[i] > high2 {
                high2 = slice[i];
            }
        }

        total += ((high - 0x30) * 10 + high2 - 0x30) as u64;
    }

    total
}

fn p2(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let mut total = 0;
    for line_ref in input {
        let line = line_ref.as_ref();
        let mut line_raw = line.as_bytes();
        let mut digits = [0u8; 12];

        for digit_idx in 0..digits.len() {
            let slice = &line_raw[..line_raw.len() - (digits.len() - 1 - digit_idx)];
            let mut idx = 0;
            let mut high = slice[0];
            for i in 1..slice.len() {
                if slice[i] > high {
                    high = slice[i];
                    idx = i;
                }
            }

            line_raw = &line_raw[idx + 1..];
            digits[digit_idx] = high - 0x30;
        }

        digits.reverse();
        total += digits
            .into_iter()
            .enumerate()
            .fold(0u64, |a, (idx, b)| a + (10u64.pow(idx as u32) * b as u64))
            as u64
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_reference() {
        let input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];

        assert_eq!(357, p1(input.iter()));
    }

    #[test]
    fn p2_reference() {
        let input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];

        assert_eq!(3121910778619, p2(input.iter()));
    }
}
