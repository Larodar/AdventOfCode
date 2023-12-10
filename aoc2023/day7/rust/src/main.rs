use std::cmp::Ordering;
use std::io::{stdin, BufRead};
use std::str::FromStr;

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

pub fn p1(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let game = input.map(|l| Hand::from_str(l.as_ref()).unwrap());
}

struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl Hand {
    fn compare(&self, other: &Hand) -> std::cmp::Ordering {
        for idx in 0..5 {
            let ord = self.cards[idx].cmp(&other.cards[idx]);
            if ord != Ordering::Equal {
                return ord;
            }
        }

        Ordering::Equal
    }
}

impl FromStr for Hand {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s.split_once(' ').unwrap();
        let bytes = cards_str.as_bytes();
        let mut cards = [
            bytes[0].into(),
            bytes[1].into(),
            bytes[2].into(),
            bytes[3].into(),
            bytes[4].into(),
        ];
        Ok(Hand {
            cards,
            bid: bid_str.parse().unwrap(),
        })
    }
}

#[repr(u8,)]
#[derive(Debug, PartialEq)]
enum Card {
    Two = 1,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value {
            0x32 => Card::Two,
            0x33 => Card::Three,
            0x34 => Card::Four,
            0x35 => Card::Five,
            0x36 => Card::Six,
            0x37 => Card::Seven,
            0x38 => Card::Eight,
            0x39 => Card::Nine,
            0x54 => Card::Ten,
            0x4A => Card::J,
            0x51 => Card::Q,
            0x4B => Card::K,
            0x41 => Card::A,
            _ => unreachable!(),
        }
    }
}

pub fn p2(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    0
}
