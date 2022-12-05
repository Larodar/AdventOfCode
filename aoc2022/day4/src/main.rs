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

fn p1<I: Iterator<Item = u8>>(iter: I) -> u64 {
    let mut ret = 0;
    let mut line_buffer = Vec::with_capacity(64);
    let mut peekable = iter.peekable();
    let peekable_ref = &mut peekable;
    while let Some(_) = peekable_ref.peek() {
        // read lines
        peekable_ref.take_while(|b| *b != b'\n').fold(
            &mut line_buffer,
            |target: &mut Vec<u8>, b| {
                target.push(b);
                target
            },
        );

        let line_str = str::from_utf8(&line_buffer[..]).unwrap();
        let mut parts = line_str.split(',');
        let one: SectionSpan = parts.next().unwrap().parse().unwrap();
        let two: SectionSpan = parts.next().unwrap().parse().unwrap();

        if one.contains(&two) || two.contains(&one) {
            ret += 1;
        }

        line_buffer.clear();
    }

    ret
}

fn p2<I: Iterator<Item = u8>>(iter: I) -> u64 {
    let mut ret = 0;
    let mut line_buffer = Vec::with_capacity(64);
    let mut peekable = iter.peekable();
    let peekable_ref = &mut peekable;
    while let Some(_) = peekable_ref.peek() {
        // read lines
        peekable_ref.take_while(|b| *b != b'\n').fold(
            &mut line_buffer,
            |target: &mut Vec<u8>, b| {
                target.push(b);
                target
            },
        );

        let line_str = str::from_utf8(&line_buffer[..]).unwrap();
        let mut parts = line_str.split(',');
        let one: SectionSpan = parts.next().unwrap().parse().unwrap();
        let two: SectionSpan = parts.next().unwrap().parse().unwrap();

        if one.overlaps(&two) {
            println!("{} {}", one, two);
            ret += 1;
        }

        line_buffer.clear();
    }

    ret
}

#[derive(Debug)]
struct SectionSpan(u32, u32);

impl SectionSpan {
    fn contains(&self, other: &SectionSpan) -> bool {
        self.0 >= other.0 && self.1 <= other.1
    }

    fn overlaps(&self, other: &SectionSpan) -> bool {
        self.1 >= other.0 && self.0 <= other.1
    }
}

impl Display for SectionSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

impl FromStr for SectionSpan {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        Ok(SectionSpan(
            parts.next().unwrap().parse()?,
            parts.next().unwrap().parse()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = b"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .to_vec();
        assert_eq!(2, p1(input.into_iter()));
    }

    #[test]
    fn p2_test() {
        let input = b"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .to_vec();
        assert_eq!(4, p2(input.into_iter()));
    }
}
