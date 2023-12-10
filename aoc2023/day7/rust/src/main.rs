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

pub fn p1(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let mut game = input
        .map(|l| Hand::from_str(l.as_ref()).unwrap())
        .collect::<Vec<_>>();
    game.sort();
    game.into_iter()
        .enumerate()
        .map(|(idx, item)| item.bid * (idx as u64 + 1))
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl Hand {
    fn strength(&self) -> Strength {
        let mut kind_count = [0u8; 13];
        for c in self.cards.iter() {
            kind_count[(*c as usize) - 2] += 1;
        }

        match kind_count.iter().max() {
            Some(1) => Strength::HighCard,
            Some(2) if kind_count.iter().filter(|slot| **slot == 2).count() == 2 => {
                Strength::TwoPair
            }
            Some(2) => Strength::OnePair,
            Some(3) if kind_count.iter().find(|slot| **slot == 2).is_some() => Strength::FullHouse,
            Some(3) => Strength::ThreeOfAKind,
            Some(4) => Strength::FourOfAKind,
            Some(5) => Strength::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        let ord = self.strength().cmp(&other.strength());
        if ord != Ordering::Equal {
            return ord;
        }

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
        let cards = [
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

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum Card {
    Two = 2,
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

impl From<usize> for Card {
    fn from(value: usize) -> Self {
        match value {
            0x00 => Card::Two,
            0x01 => Card::Three,
            0x02 => Card::Four,
            0x03 => Card::Five,
            0x04 => Card::Six,
            0x05 => Card::Seven,
            0x06 => Card::Eight,
            0x07 => Card::Nine,
            0x08 => Card::Ten,
            0x09 => Card::J,
            0x0A => Card::Q,
            0x0B => Card::K,
            0x0C => Card::A,
            val => panic!("Unknown value for card {val}"),
        }
    }
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
    let mut game = input
        .map(|l| Hand2::from_str(l.as_ref()).unwrap())
        .collect::<Vec<_>>();
    game.sort();
    game.into_iter()
        .enumerate()
        .map(|(idx, item)| item.bid * (idx as u64 + 1))
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Hand2 {
    cards: [Card2; 5],
    bid: u64,
}

impl Hand2 {
    fn strength(&self) -> Strength {
        let mut kind_count = [0u8; 13];
        for c in self.cards.iter() {
            kind_count[(*c as usize) - 1] += 1;
        }

        let joker_count = kind_count[0];
        // skip jokers
        let max_count = kind_count.iter().skip(1).max().unwrap();
        match (max_count, joker_count + max_count) {
            (_, 5) => Strength::FiveOfAKind,
            (_, 4) => Strength::FourOfAKind,
            (1, 1) => Strength::HighCard,
            (2, 2) if kind_count.iter().skip(1).filter(|slot| **slot == 2).count() == 2 => {
                Strength::TwoPair
            }
            (2, 3) if kind_count.iter().skip(1).filter(|slot| **slot == 2).count() == 2 => {
                Strength::FullHouse
            }
            (1 | 2, 2) => Strength::OnePair,
            (3, 3) if kind_count.iter().skip(1).find(|slot| **slot == 2).is_some() => {
                Strength::FullHouse
            }
            (1 | 2 | 3, 3) => Strength::ThreeOfAKind,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Hand2 {
    fn cmp(&self, other: &Hand2) -> std::cmp::Ordering {
        let ord = self.strength().cmp(&other.strength());
        if ord != Ordering::Equal {
            return ord;
        }

        for idx in 0..5 {
            let ord = self.cards[idx].cmp(&other.cards[idx]);
            if ord != Ordering::Equal {
                return ord;
            }
        }

        Ordering::Equal
    }
}

impl FromStr for Hand2 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s.split_once(' ').unwrap();
        let bytes = cards_str.as_bytes();
        let cards = [
            bytes[0].into(),
            bytes[1].into(),
            bytes[2].into(),
            bytes[3].into(),
            bytes[4].into(),
        ];
        Ok(Hand2 {
            cards,
            bid: bid_str.parse().unwrap(),
        })
    }
}

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum Card2 {
    J = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Q,
    K,
    A,
}

impl From<usize> for Card2 {
    fn from(value: usize) -> Self {
        match value {
            0x00 => Card2::J,
            0x01 => Card2::Two,
            0x02 => Card2::Three,
            0x03 => Card2::Four,
            0x04 => Card2::Five,
            0x05 => Card2::Six,
            0x06 => Card2::Seven,
            0x07 => Card2::Eight,
            0x08 => Card2::Nine,
            0x09 => Card2::Ten,
            0x0A => Card2::Q,
            0x0B => Card2::K,
            0x0C => Card2::A,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for Card2 {
    fn from(value: u8) -> Self {
        match value {
            0x32 => Card2::Two,
            0x33 => Card2::Three,
            0x34 => Card2::Four,
            0x35 => Card2::Five,
            0x36 => Card2::Six,
            0x37 => Card2::Seven,
            0x38 => Card2::Eight,
            0x39 => Card2::Nine,
            0x54 => Card2::Ten,
            0x4A => Card2::J,
            0x51 => Card2::Q,
            0x4B => Card2::K,
            0x41 => Card2::A,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strength() {
        let h = Hand {
            cards: [Card::Two, Card::Three, Card::Four, Card::Five, Card::Six],
            bid: 0,
        };

        assert_eq!(Strength::HighCard, h.strength())
    }

    #[test]
    fn sample() {
        let input = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];

        assert_eq!(6440, p1(input.into_iter()));
    }

    #[test]
    fn sample_with_equal_hands() {
        let input = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];

        assert_eq!(7719, p1(input.into_iter()));
    }

    #[test]
    fn sample_p2() {
        let input = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];

        assert_eq!(5905, p2(input.into_iter()));
    }
}
