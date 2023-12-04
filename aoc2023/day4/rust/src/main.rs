use std::io::{stdin, BufRead};

fn main() {
    match std::env::args().nth(1).map(|s| s.parse::<u32>().unwrap()) {
        Some(1) => println!(
            "{}",
            p1::<10, 25>(
                stdin()
                    .lock()
                    .lines()
                    .map(|line_result| line_result.unwrap())
            )
        ),
        Some(2) => println!(
            "{}",
            p2::<10, 25>(
                stdin()
                    .lock()
                    .lines()
                    .map(|line_result| line_result.unwrap())
            )
        ),
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}

fn p1<const W: usize, const N: usize>(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let mut score = 0;
    for card in input
        .into_iter()
        .map(|l| Card::<W, N>::from_line(l.as_ref()))
    {
        let mut card_score = 0;
        for n in card.numbers.iter() {
            card_score = match (card.winning.contains(n), card_score) {
                (true, 0) => 1,
                (true, _) => 2 * card_score,
                (false, _) => card_score,
            }
        }

        score += card_score
    }

    score
}

fn p2<const W: usize, const N: usize>(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    // Vec<(matches, count)>
    let mut cards = input
        .into_iter()
        .map(|l| Card::<W, N>::from_line(l.as_ref()))
        .map(|c| {
            (
                // calc number of matches
                c.numbers.iter().filter(|n| c.winning.contains(n)).count() as u64,
                1,
            )
        })
        .collect::<Vec<_>>();

    for idx in 0..cards.len() {
        let (matches, count) = cards[idx];
        let to_update = std::cmp::min(matches, (cards.len() - idx) as u64);
        for i in 1..=to_update {
            cards[idx + i as usize].1 += count;
        }
    }

    cards.into_iter().map(|(_, count)| count).sum()
}

#[derive(Debug)]
struct Card<const W: usize, const N: usize> {
    winning: [u32; W],
    numbers: [u32; N],
}

impl<const W: usize, const N: usize> Card<W, N> {
    fn from_line(l: &str) -> Card<W, N> {
        let (_, rest) = l.split_once(':').unwrap();
        let (winning_str, numbers_str) = rest.split_once('|').unwrap();

        let mut winning = [0; W];
        let mut numbers = [0; N];
        let mut winning_iter = winning_str
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap());
        for idx in 0..winning.len() {
            winning[idx] = winning_iter.next().unwrap();
        }

        let mut number_iter = numbers_str
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap());
        for idx in 0..numbers.len() {
            numbers[idx] = number_iter.next().unwrap();
        }

        Card { winning, numbers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_game() {
        let input = vec!["Card   1: 44 22 11 15 37 50  3 90 60 34 | 35 60 76  3 21 84 45 52 15 72 13 31 90  6 37 44 34 53 68 22 50 38 67 11 55"];

        assert!(2 < p1::<10, 25>(input.into_iter()));
    }

    #[test]
    fn should_calc_p2() {
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        assert_eq!(30, p2::<5, 8>(input.into_iter()));
    }
}
