use std::collections::HashSet;
use std::io::{stdin, Read};

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
    let mut line_set = HashSet::with_capacity(64);
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

        let mid = line_buffer.len() / 2;
        for entry in &line_buffer[..mid] {
            _ = line_set.insert(*entry);
        }

        for entry in &line_buffer[mid..] {
            if line_set.contains(entry) {
                ret += get_priority(*entry);
                break;
            }
        }
        line_buffer.clear();
        line_set.clear();
    }

    ret
}

fn p2<I: Iterator<Item = u8>>(iter: I) -> u64 {
    let mut ret = 0;
    let mut lines: Vec<Vec<u8>> = vec![
        Vec::with_capacity(64),
        Vec::with_capacity(64),
        Vec::with_capacity(64),
    ];
    let mut line_sets = vec![HashSet::with_capacity(64), HashSet::with_capacity(64)];
    let mut peekable = iter.peekable();
    let peekable_ref = &mut peekable;

    while let Some(_) = peekable_ref.peek() {
        // read lines
        for i in 0..lines.len() {
            peekable_ref.take_while(|b| *b != b'\n').fold(
                &mut lines[i],
                |target: &mut Vec<u8>, b| {
                    target.push(b);
                    target
                },
            );
        }

        // build sets
        for i in 0..line_sets.len() {
            for b in lines[i].iter() {
                _ = line_sets[i].insert(*b);
            }
        }

        // search
        for b in lines[2].iter() {
            if line_sets[0].contains(b) && line_sets[1].contains(b) {
                ret += get_priority(*b);
                break;
            }
        }

        line_sets.iter_mut().for_each(|entry| entry.clear());
        lines.iter_mut().for_each(|entry| entry.clear());
    }

    ret
}

fn get_priority(b: u8) -> u64 {
    (if b > 64 && b < 91 {
        b - 64 + 26
    } else {
        b - 96
    }) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_prio_test() {
        assert_eq!(1, get_priority(b'a'));
        assert_eq!(26, get_priority(b'z'));

        assert_eq!(27, get_priority(b'A'));
        assert_eq!(52, get_priority(b'Z'));
    }

    #[test]
    fn p1_test() {
        let input = b"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_vec();
        assert_eq!(157, p1(input.into_iter()));
    }

    #[test]
    fn p2_test() {
        let input = b"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_vec();
        assert_eq!(70, p2(input.into_iter()));
    }
}
