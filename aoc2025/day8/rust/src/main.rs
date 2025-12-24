use std::{fmt::Display, io::stdin, num::ParseIntError, str::FromStr};

fn main() {
    match std::env::args().nth(1).map(|s| s.parse::<u32>().unwrap()) {
        Some(1) => println!(
            "{}",
            p1(stdin().lines().map(|line_result| line_result.unwrap()))
        ),
        Some(2) => println!(
            "{}",
            p2(stdin().lines().map(|line_result| line_result.unwrap()))
        ),
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}

#[cfg(test)]
const CONNECTION_COUNT: usize = 10;
#[cfg(not(test))]
const CONNECTION_COUNT: usize = 1000;

fn p1(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let boxes: Vec<JBox> = input.map(|l| l.as_ref().parse().unwrap()).collect();

    let mut distances = boxes
        .iter()
        .enumerate()
        .map(|(idx, lhs)| {
            boxes[idx + 1..].iter().map(|rhs| Connection {
                lhs: *lhs,
                rhs: *rhs,
                dist: lhs.eucl_dist(rhs),
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    distances.sort_unstable_by_key(|p| p.dist);

    let mut circuits: Vec<Vec<JBox>> = vec![];
    let mut pairs = CONNECTION_COUNT;
    for pair in distances.iter().take(CONNECTION_COUNT) {
        if pairs == 0 {
            break;
        }

        let mut lhs_idx = None;
        let mut rhs_idx = None;
        for i in 0..circuits.len() {
            if circuits[i].contains(&pair.lhs) {
                assert!(lhs_idx.is_none());
                lhs_idx = Some(i);
            }

            if circuits[i].contains(&pair.rhs) {
                assert!(rhs_idx.is_none());
                rhs_idx = Some(i);
            }
        }

        match (lhs_idx, rhs_idx) {
            (Some(l), Some(r)) => {
                if l == r {
                    continue;
                }

                // merge
                let first = l.min(r);
                let second = l.max(r);
                let connected = circuits.swap_remove(second);
                circuits[first].extend_from_slice(&connected);
            }
            (None, Some(r)) => {
                circuits[r].push(pair.lhs);
            }
            (Some(l), None) => {
                circuits[l].push(pair.rhs);
            }
            (None, None) => {
                circuits.push(vec![pair.rhs, pair.lhs]);
            }
        }
        pairs -= 1;
    }

    let (first, second, third) = circuits
        .iter()
        .map(|c| c.len())
        .take_while(|v| *v != 0)
        .fold((0, 0, 0), |acc, c| {
            if c > acc.0 {
                (c, acc.0, acc.1)
            } else if c > acc.1 {
                (acc.0, c, acc.1)
            } else if c > acc.2 {
                (acc.0, acc.1, c)
            } else {
                acc
            }
        });

    (first * second * third) as u64
}

fn p2(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let boxes: Vec<JBox> = input.map(|l| l.as_ref().parse().unwrap()).collect();

    let mut distances = boxes
        .iter()
        .enumerate()
        .map(|(idx, lhs)| {
            boxes[idx + 1..].iter().map(|rhs| Connection {
                lhs: *lhs,
                rhs: *rhs,
                dist: lhs.eucl_dist(rhs),
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    distances.sort_unstable_by_key(|p| p.dist);

    let mut circuits: Vec<Vec<JBox>> = vec![];
    let mut iter = distances.iter();
    let terminator = loop {
        let pair = iter.next().unwrap();
        let mut lhs_idx = None;
        let mut rhs_idx = None;
        for i in 0..circuits.len() {
            if circuits[i].contains(&pair.lhs) {
                assert!(lhs_idx.is_none());
                lhs_idx = Some(i);
            }

            if circuits[i].contains(&pair.rhs) {
                assert!(rhs_idx.is_none());
                rhs_idx = Some(i);
            }
        }

        match (lhs_idx, rhs_idx) {
            (Some(l), Some(r)) => {
                if l == r {
                    continue;
                }

                // merge
                let first = l.min(r);
                let second = l.max(r);
                let connected = circuits.swap_remove(second);
                circuits[first].extend_from_slice(&connected);
            }
            (None, Some(r)) => {
                circuits[r].push(pair.lhs);
            }
            (Some(l), None) => {
                circuits[l].push(pair.rhs);
            }
            (None, None) => {
                circuits.push(vec![pair.rhs, pair.lhs]);
            }
        }

        if circuits.len() == 1 && circuits[0].len() == boxes.len() {
            break pair;
        }
    };

    terminator.rhs.x * terminator.lhs.x
}

#[derive(Debug, PartialEq)]
struct Connection {
    lhs: JBox,
    rhs: JBox,
    dist: u64,
}

impl Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}: {}", self.rhs, self.lhs, self.dist)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct JBox {
    x: u64,
    y: u64,
    z: u64,
}

impl JBox {
    fn eucl_dist(&self, other: &JBox) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

impl Display for JBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl FromStr for JBox {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split.next().unwrap().parse()?;
        let y = split.next().unwrap().parse()?;
        let z = split.next().unwrap().parse()?;
        Ok(JBox { x, y, z })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1_reference() {
        let input = vec![
            "162,817,812",
            "57,618,57",
            "906,360,560",
            "592,479,940",
            "352,342,300",
            "466,668,158",
            "542,29,236",
            "431,825,988",
            "739,650,466",
            "52,470,668",
            "216,146,977",
            "819,987,18",
            "117,168,530",
            "805,96,715",
            "346,949,466",
            "970,615,88",
            "941,993,340",
            "862,61,35",
            "984,92,344",
            "425,690,689",
        ];

        assert_eq!(40, p1(input.iter()));
    }

    #[test]
    fn p2_reference() {
        let input = vec![
            "162,817,812",
            "57,618,57",
            "906,360,560",
            "592,479,940",
            "352,342,300",
            "466,668,158",
            "542,29,236",
            "431,825,988",
            "739,650,466",
            "52,470,668",
            "216,146,977",
            "819,987,18",
            "117,168,530",
            "805,96,715",
            "346,949,466",
            "970,615,88",
            "941,993,340",
            "862,61,35",
            "984,92,344",
            "425,690,689",
        ];

        assert_eq!(25272, p2(input.iter()));
    }
}
