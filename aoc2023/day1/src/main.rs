use std::{
    io::{stdin, Read, BufRead},
};

fn main() {
    let mut buf = String::new();
    _ = stdin().read_to_string(&mut buf).unwrap();

    match std::env::args()
        .nth(1)
        .map(|s| s.parse::<u32>().unwrap())
    {
            Some(1) => println!("{}", p1(stdin().lock().lines().map(|line_result| line_result.unwrap()))),
        Some(2) => println!("{}", p2(stdin().lock().lines().map(|line_result| line_result.unwrap()))),
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}

fn p1(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let mut total = 0u64;
    for l in input {
        let bytes = l.as_ref().as_bytes();
        let first = bytes.iter().find(|&&b| (0x30..=0x39).contains(&b)).unwrap();
        let last = bytes
            .iter()
            .rev()
            .find(|&&b| (0x30..0x39).contains(&b))
            .unwrap();
        let buf = [*first, *last];
        let string = std::str::from_utf8(&buf[..]).unwrap();

        total += string.parse::<u64>().unwrap();
    }

    total
}

fn p2(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let mut total = 0u64;
    for l in input {
        let bytes = l.as_ref().as_bytes();
        let (first, mut remainder) = find_digit(bytes);
        let mut last = first;
        loop {
            let (result, rem) = find_digit(remainder);
            remainder = rem;
            if result != 0 {
                last = result;
            }

            if rem.is_empty() {
                break;
            }
        }
        let buf = [first, last];
        let string = std::str::from_utf8(&buf[..]).unwrap();
        total += string.parse::<u64>().unwrap();
    }

    total
}

fn find_digit(bytes: &[u8]) -> (u8, &[u8]) {
    let len = bytes.len();
    for i in 0..len {
        let b = bytes[i];
        if (0x30..=0x39).contains(&b) {
            return (b, &bytes[i+1..]);
        }

        match b {
            0x6F if len - i > 2         // o
                && bytes[i + 1] == 0x6E // n
                && bytes[i + 2] == 0x65 // e
                => return (0x31, &bytes[i + 2..]), 
            0x74 if len - i > 2         // t
                && bytes[i + 1] == 0x77 // w
                && bytes[i + 2] == 0x6F // o
                => return (0x32, &bytes[i + 2..]), 
            0x74 if len - i > 4         // t
                && bytes[i + 1] == 0x68 // h
                && bytes[i + 2] == 0x72 // r
                && bytes[i + 3] == 0x65 // e
                && bytes[i + 4] == 0x65 // e
                => return (0x33, &bytes[i+4..]), 
            0x66 if len - i > 3         // f
                && bytes[i + 1] == 0x6F // o
                && bytes[i + 2] == 0x75 // u
                && bytes[i + 3] == 0x72 // r
                => return (0x34, &bytes[i+3..]),
            0x66 if len - i > 3         // f
                && bytes[i + 1] == 0x69 // i
                && bytes[i + 2] == 0x76 // v
                && bytes[i + 3] == 0x65 // e
                => return (0x35, &bytes[i+3..]),
            0x73 if len - i > 2         // s
                && bytes[i + 1] == 0x69 // i
                && bytes[i + 2] == 0x78 // x
                => return (0x36, &bytes[i+2..]),
            0x73 if len - i > 4         // s
                && bytes[i + 1] == 0x65 // e
                && bytes[i + 2] == 0x76 // v
                && bytes[i + 3] == 0x65 // e
                && bytes[i + 4] == 0x6E // n
                => return (0x37, &bytes[i+4..]),
            0x65 if len -i > 4          // e
                && bytes[i + 1] == 0x69 // i
                && bytes[i + 2] == 0x67 // g
                && bytes[i + 3] == 0x68 // h
                && bytes[i + 4] == 0x74 // t
                => return (0x38, &bytes[i+4..]),
            0x6E if len - i > 3         // n
                && bytes[i + 1] == 0x69 // i
                && bytes[i + 2] == 0x6E // n
                && bytes[i + 3] == 0x65 // e
                => return (0x39, &bytes[i+3..]),
            _ => {}
        }
    }

    (0, &[])
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
