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
fn p1(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let line_temp = input.next().unwrap();
    let line = line_temp.as_ref();
    const ID_DEL: u8 = ',' as u8;
    const RANGE_DEL: u8 = '-' as u8;
    let mut buf = [0u8; 64];

    let mut total = 0;
    for slice in line.as_bytes().split(|b| *b == ID_DEL) {
        let idx = slice.iter().position(|b| *b == RANGE_DEL).unwrap();
        let (lower, upper) = slice.split_at(idx);
        let upper = &upper[1..];
        for (idx, b) in upper.iter().enumerate() {
            buf[idx] = *b - 0x30;
        }

        let mut raw = &mut buf[..upper.len()];

        let lower: usize = std::str::from_utf8(lower).unwrap().parse().unwrap();
        let upper: usize = std::str::from_utf8(upper).unwrap().parse().unwrap();
        let count = upper - lower + 1;
        for i in 0..=count {
            if raw[0] == 0 {
                raw = &mut raw[1..];
                if raw.is_empty() {
                    break;
                }
            }

            if raw.len() & 1 == 0 {
                let split = raw.len() / 2;
                let front = &raw[..split];
                let back = &raw[split..];
                if front
                    .iter()
                    .zip(back.iter())
                    .map(|(a, b)| (a.wrapping_sub(*b)) as u64)
                    .sum::<u64>()
                    == 0
                {
                    total += upper - i;
                }
            }

            // dec number
            let mut idx = raw.len() - 1;
            loop {
                if raw[idx] == 0 {
                    raw[idx] = 9;
                    if idx == 0 {
                        break;
                    }

                    idx -= 1;
                } else {
                    raw[idx] -= 1;
                    break;
                }
            }
        }
    }

    total as u64
}

fn p2(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let line_temp = input.next().unwrap();
    let line = line_temp.as_ref();
    const ID_DEL: u8 = ',' as u8;
    const RANGE_DEL: u8 = '-' as u8;
    let mut buf = [0u8; 64];

    let mut total = 0;
    for slice in line.as_bytes().split(|b| *b == ID_DEL) {
        let idx = slice.iter().position(|b| *b == RANGE_DEL).unwrap();
        let (lower, upper) = slice.split_at(idx);
        let upper = &upper[1..];
        for (idx, b) in upper.iter().enumerate() {
            buf[idx] = *b - 0x30;
        }

        let mut raw = &mut buf[..upper.len()];

        let lower: usize = std::str::from_utf8(lower).unwrap().parse().unwrap();
        let upper: usize = std::str::from_utf8(upper).unwrap().parse().unwrap();
        let count = upper - lower;
        for i in 0..=count {
            if raw[0] == 0 {
                raw = &mut raw[1..];
                if raw.is_empty() {
                    break;
                }
            }

            if check(raw) {
                total += upper - i;
            }

            // dec number
            let mut idx = raw.len() - 1;
            loop {
                if raw[idx] == 0 {
                    if idx == 0 {
                        break;
                    }

                    raw[idx] = 9;
                    idx -= 1;
                } else {
                    raw[idx] -= 1;
                    break;
                }
            }
        }
    }

    total as u64
}

fn check(raw: &[u8]) -> bool {
    if raw.len() < 2 {
        return false;
    }

    if raw.iter().all(|b| *b == raw[0]) {
        return true;
    }

    if raw.len() > 3 {
        for window_len in 2..=raw.len() / 2 {
            if raw.len() % window_len != 0 {
                continue;
            }

            let first_window = &raw[..window_len];
            let mut start = window_len;
            let mut end = window_len + window_len;
            let mut total = 0;
            while end <= raw.len() {
                total += first_window
                    .iter()
                    .zip(raw[start..end].iter())
                    .map(|(a, b)| (a.wrapping_sub(*b)) as u64)
                    .sum::<u64>();

                start = end;
                end += window_len;
            }

            if total == 0 {
                return true;
            }
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1_single_1() {
        let input = "11-22";
        assert_eq!(33, p1(input.lines()));
    }

    #[test]
    fn p1_single_2() {
        let input = "95-115";
        assert_eq!(99, p1(input.lines()));
    }

    #[test]
    fn p1_single_3() {
        let input = "998-1012";
        assert_eq!(1010, p1(input.lines()));
    }

    #[test]
    fn p1_reference() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(1227775554, p1(input.lines()));
    }

    #[test]
    fn p2_reference() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(4174379265, p2(input.lines()));
    }

    #[test]
    fn p2_single() {
        let input = "2121212118-2121212124";
        assert_eq!(2121212121, p2(input.lines()));
    }

    #[test]
    fn p2_single_1() {
        let input = "11-22";
        assert_eq!(33, p2(input.lines()));
    }

    #[test]
    fn p2_single_2() {
        let input = "95-115";
        assert_eq!(210, p2(input.lines()));
    }

    #[test]
    fn p2_single_3() {
        let input = "1188511880-1188511890";
        assert_eq!(1188511885, p2(input.lines()));
    }

    #[test]
    fn p2_single_4() {
        let input = "222220-222224";
        assert_eq!(222222, p2(input.lines()));
    }

    #[test]
    fn test_check() {
        let input = vec![2, 2, 2, 2, 2, 2];
        assert_eq!(true, check(&input[..]));
        let input = vec![2, 2, 2, 2, 2, 3];
        assert_eq!(false, check(&input[..]));
        let input = vec![2, 2, 2, 2, 2, 0];
        assert_eq!(false, check(&input[..]));
    }
}
