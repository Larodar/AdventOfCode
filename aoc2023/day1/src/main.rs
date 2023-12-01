use std::{
    io::{stdin, Read},
};

fn main() {
    let mut buf = String::new();
    _ = stdin().read_to_string(&mut buf).unwrap();

    match std::env::args()
        .into_iter()
        .skip(1)
        .next()
        .map(|s| s.parse::<u32>().unwrap())
    {
        Some(1) => println!("{}", p1(&buf[..])),
        Some(2) => println!("{}", p2(&buf[..])),
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}

fn p1(input: &str) -> u64 {
    let mut total = 0u64;
    for l in input.lines() {
        let bytes = l.as_bytes();
        let first = bytes.iter().find(|&&b| b >= 0x30 && b < 0x40).unwrap();
        let last = bytes
            .iter()
            .rev()
            .find(|&&b| b >= 0x30 && b < 0x40)
            .unwrap();
        let buf = [*first, *last];
        let string = std::str::from_utf8(&buf[..]).unwrap();

        total += string.parse::<u64>().unwrap();
    }

    total
}

fn p2(input: &str) -> u64 {
    let mut total = 0u64;
    for l in input.lines() {
        let bytes = l.as_bytes();
        let (first, mut remainder) = find_digit(bytes);
        let mut last = first;
        loop {
            let (result, rem) = dbg!(find_digit(remainder));
            remainder = rem;
            if result != 0 {
                last = result;
            }

            if rem.is_empty() {
                break;
            }
        }
        let buf = dbg!([first, last]);
        let string = std::str::from_utf8(&buf[..]).unwrap();

        total += string.parse::<u64>().unwrap();
    }

    total
}

fn find_digit(bytes: &[u8]) -> (u8, &[u8]) {
    let len = bytes.len();
    for i in 0..len {
        let b = bytes[i];
        if b > 0x30 && b < 0x40 {
            return (b, &bytes[i+1..]);
        }

        match b.try_into() {
            Ok(FirstLetter::O) if len - i > 2
                && bytes[i + 1] == 0x6E // n
                && bytes[i + 2] == 0x65 // e
                => return (0x31, &bytes[i + 2..]), 
            Ok(FirstLetter::T) if len - i > 2
                && bytes[i + 1] == 0x77 // w
                && bytes[i + 2] == 0x6F // o
                => return (0x32, &bytes[i + 2..]), 
            Ok(FirstLetter::T) if len - i > 4
                && bytes[i + 1] == 0x68 // h
                && bytes[i + 2] == 0x72 // r
                && bytes[i + 3] == 0x65 // e
                && bytes[i + 4] == 0x65 // e
                => return (0x33, &bytes[i+4..]), 
            Ok(FirstLetter::F) if len - i > 3
                && bytes[i + 1] == 0x6F // o
                && bytes[i + 2] == 0x75 // u
                && bytes[i + 3] == 0x72 // r
                => return (0x34, &bytes[i+3..]),
            Ok(FirstLetter::F) if len - i > 3
                && bytes[i + 1] == 0x69 // i
                && bytes[i + 2] == 0x76 // v
                && bytes[i + 3] == 0x65 // e
                => return (0x35, &bytes[i+3..]),
            Ok(FirstLetter::S) if len - i > 2
                && bytes[i + 1] == 0x69 // i
                && bytes[i + 2] == 0x78 // x
                => return (0x36, &bytes[i+2..]),
            Ok(FirstLetter::S) if len - i > 4
                && bytes[i + 1] == 0x65 // e
                && bytes[i + 2] == 0x76 // v
                && bytes[i + 3] == 0x65 // e
                && bytes[i + 4] == 0x6E // n
                => return (0x37, &bytes[i+4..]),
            Ok(FirstLetter::E) if len -i > 4
                && bytes[i + 1] == 0x69 // i
                && bytes[i + 2] == 0x67 // g
                && bytes[i + 3] == 0x68 // h
                && bytes[i + 4] == 0x74 // t
                => return (0x38, &bytes[i+4..]),
            Ok(FirstLetter::N) if len - i > 3
                && bytes[i + 1] == 0x69 // i
                && bytes[i + 2] == 0x6E // n
                && bytes[i + 3] == 0x65 // e
                => return (0x39, &bytes[i+3..]),
            _ => {}
        }
    }

    (0, &[])
}

enum FirstLetter {
    O,
    T,
    F,
    S,
    E,
    N,
    Z,
}

impl TryFrom<u8> for FirstLetter {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x6F | 0x4F => Ok(FirstLetter::O),
            0x74 | 0x54 => Ok(FirstLetter::T),
            0x66 | 0x46 => Ok(FirstLetter::F),
            0x73 | 0x53 => Ok(FirstLetter::S),
            0x65 | 0x45 => Ok(FirstLetter::E),
            0x6E | 0x4E => Ok(FirstLetter::N),
            0x7A | 0x5A => Ok(FirstLetter::Z),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::find_digit;

    #[test]
    fn should_find_digits() {
        let line = "two1nine".as_bytes();
        let (result, rest) = find_digit(line);
        assert_eq!(0x32, result);
        let (result, rest) = find_digit(rest);
        assert_eq!(0x31, result);
        let (result, rest) = find_digit(rest);
        assert_eq!(0x39, result);
        assert_eq!(1, rest.len());
        assert_eq!((0u8, &[][..]), find_digit(rest));
    }

    #[test]
    fn should_find_digits_no_digit_byte() {
        let line = "eightwothree".as_bytes();
        let (result, rest) = find_digit(line);
        assert_eq!(0x38, result);
        let (result, rest) = find_digit(rest);
        assert_eq!(0x32, result);
        let (result, rest) = find_digit(rest);
        assert_eq!(0x33, result);
        assert_eq!(1, rest.len());
        assert_eq!((0u8, &[][..]), find_digit(rest));
    }

    #[test]
    fn should_find_digits_with_clutter() {
        let line = "abcone2threexyz".as_bytes();
        let (result, rest) = find_digit(line);
        assert_eq!(0x31, result);
        let (result, rest) = find_digit(rest);
        assert_eq!(0x32, result);
        let (result, rest) = find_digit(rest);
        assert_eq!(0x33, result);
        assert_eq!(4, rest.len());
        assert_eq!((0u8, &[][..]), find_digit(rest));
    }

    #[test]
    fn should_find_digits_with_clutter_interleaved() {
        let line = "xtwone3four".as_bytes();
        let (result, rest) = find_digit(line);
        assert_eq!(0x32, result);
        let (result, rest) = find_digit(rest);
        assert_eq!(0x31, result);
        let (result, rest) = find_digit(rest);
        assert_eq!(0x33, result);
        let (result, rest) = find_digit(rest);
        assert_eq!(0x34, result);
        assert_eq!(1, rest.len());
        assert_eq!((0u8, &[][..]), find_digit(rest));
    }

    #[test]
    fn should_find_digits_with_end_and_start_digits() {
        let line = "4nineeightseven2".as_bytes();
        let (result, rest) = find_digit(line);
        assert_eq!(0x34, result);
        let (result, rest) = find_digit(rest);
        assert_eq!(0x39, result);
        let (result, rest) = find_digit(rest);
        assert_eq!(0x38, result);
        eprintln!("{}", std::str::from_utf8(rest).unwrap());
        let (result, rest) = find_digit(rest);
        assert_eq!(0x37, result);
        let (result, rest) = find_digit(rest);
        assert_eq!(0x32, result);
        assert_eq!(0, rest.len());
        assert_eq!((0u8, &[][..]), find_digit(rest));
    }
}
