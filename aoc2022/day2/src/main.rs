#![feature(portable_simd)]
#![feature(iter_next_chunk)]

use std::{
    intrinsics::transmute,
    io::{stdin, Read},
    simd::{u8x32, SimdUint},
    time::Instant,
};

fn main() {
    match std::env::args()
        .into_iter()
        .skip(1)
        .next()
        .map(|s| s.parse::<u32>().unwrap())
    {
        Some(1) => {
            let start = Instant::now();
            let result = p1(stdin().bytes().map(|b| b.unwrap()));
            println!("{}", start.elapsed().as_micros());
            println!("Total score: {}", result)
        }
        Some(2) => println!("Total score: {}", p2(stdin().bytes().map(|b| b.unwrap()))),
        Some(3) => {
            let start = Instant::now();
            let result = simd_approach(stdin().bytes().map(|b| b.unwrap()));
            println!("{}", start.elapsed().as_micros());
            println!("Total score: {}", result)
        }
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}

fn p1<I: Iterator<Item = u8>>(iter: I) -> u64 {
    RoundsIter::new(iter.filter(|b| *b > 64 && *b < 91)).fold(0u64, |acc, item| acc + item.play())
}

fn p2<I: Iterator<Item = u8>>(iter: I) -> u64 {
    RoundsIter::new(iter.filter(|b| *b > 64 && *b < 91))
        .fold(0u64, |acc, item| acc + item.play_p2())
}

fn simd_approach<I: Iterator<Item = u8>>(mut iter: I) -> u64 {
    let result_filter_mask = u8x32::from_array([
        0x0, 0x0, 0x0, 0xFF, 0x0, 0x0, 0x0, 0xFF, 0x0, 0x0, 0x0, 0xFF, 0x0, 0x0, 0x0, 0xFF, 0x0,
        0x0, 0x0, 0xFF, 0x0, 0x0, 0x0, 0xFF, 0x0, 0x0, 0x0, 0xFF, 0x0, 0x0, 0x0, 0xFF,
    ]);
    let mut total = 0;
    loop {
        match iter.next_chunk::<32>() {
            Ok(chunk) => {
                let mut v = u8x32::from_slice(&chunk);
                v[3] = simd_play(v[0], v[2]);
                v[7] = simd_play(v[4], v[6]);
                v[11] = simd_play(v[8], v[10]);
                v[15] = simd_play(v[12], v[14]);
                v[19] = simd_play(v[16], v[18]);
                v[23] = simd_play(v[20], v[22]);
                v[27] = simd_play(v[24], v[26]);
                v[31] = simd_play(v[28], v[30]);
                total += (v & result_filter_mask).reduce_sum() as u64;
            }
            Err(remaining) => {
                return RoundsIter::new(remaining.filter(|b| *b > 64 && *b < 91))
                    .fold(total, |acc, item| acc + item.play());
            }
        }
    }
}

struct RoundsIter<I: Iterator<Item = u8>> {
    inner: I,
}

impl<I: Iterator<Item = u8>> RoundsIter<I> {
    pub fn new(iter: I) -> RoundsIter<I> {
        RoundsIter { inner: iter }
    }
}

impl<I: Iterator<Item = u8>> Iterator for RoundsIter<I> {
    type Item = Round;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(them_raw) = self.inner.next() {
            let them = Them::from(them_raw);
            let us = Us::from(self.inner.next().unwrap());
            Some(Round(them, us))
        } else {
            None
        }
    }
}

fn simd_play(them: u8, us: u8) -> u8 {
    match them {
        65 => match us {
            88 => 4,
            89 => 8,
            90 => 3,
            _ => panic!(),
        },
        66 => match us {
            88 => 1,
            89 => 5,
            90 => 9,
            _ => panic!(),
        },
        67 => match us {
            88 => 7,
            89 => 2,
            90 => 6,
            _ => panic!(),
        },
        _ => panic!(),
    }
}

#[derive(Debug)]
struct Round(Them, Us);

impl Round {
    #[inline(never)]
    fn play(&self) -> u64 {
        match self.0 {
            Them::Rock => match self.1 {
                Us::X => 4,
                Us::Y => 8,
                Us::Z => 3,
            },
            Them::Paper => match self.1 {
                Us::X => 1,
                Us::Y => 5,
                Us::Z => 9,
            },
            Them::Scissors => match self.1 {
                Us::X => 7,
                Us::Y => 2,
                Us::Z => 6,
            },
        }
    }

    fn play_p2(&self) -> u64 {
        match self.1 {
            Us::X => match self.0 {
                Them::Rock => 3,
                Them::Paper => 1,
                Them::Scissors => 2,
            },
            Us::Y => {
                (match self.0 {
                    Them::Rock => 1,
                    Them::Paper => 2,
                    Them::Scissors => 3,
                }) + 3
            }
            Us::Z => {
                (match self.0 {
                    Them::Rock => 2,
                    Them::Paper => 3,
                    Them::Scissors => 1,
                }) + 6
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Us {
    /// Rock, Lose
    X = 88,
    /// Paper, Draw
    Y,
    /// Scissors, Win
    Z,
}

impl From<u8> for Us {
    fn from(value: u8) -> Self {
        unsafe { transmute(value) }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Them {
    /// Rock
    Rock = 65,
    /// Paper
    Paper,
    /// Scissors
    Scissors,
}

impl From<u8> for Them {
    fn from(value: u8) -> Self {
        unsafe { transmute(value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "A Y\nB X\nC Z\n".bytes();
        assert_eq!(p1(input), 15);
    }

    #[test]
    fn test_p2() {
        let input = "A Y\nB X\nC Z\n".bytes();
        assert_eq!(p2(input), 12);
    }
}
