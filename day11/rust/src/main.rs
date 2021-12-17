#![feature(portable_simd)]

use std::io::BufRead;
use std::ops::BitAnd;
use std::simd::mask8x32;
use std::simd::u8x32;

fn main() {
    let mut grid = read_input();
    grid.increase_energy();
}

fn read_input() -> Grid {
    println!("reading input");
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut entries = lines
        .next()
        .unwrap()
        .unwrap()
        .bytes()
        .map(|b| b - 0x30)
        .collect::<Vec<_>>();
    let side_length = entries.len();

    for line in lines {
        line.unwrap()
            .bytes()
            .map(|b| b - 0x30)
            .for_each(|v| entries.push(v));
    }

    Grid::new(entries, side_length)
}

fn step(mut grid: Grid) {
    // phase 1: increase energy levels
    grid.increase_energy();
}

struct Octopus(u8, bool);

#[derive(Debug)]
struct Grid {
    side_length: usize,
    entries: Vec<u8>,
    flashed: Vec<bool>,
}

impl Grid {
    pub fn new(entries: Vec<u8>, side_length: usize) -> Grid {
        let mut flashed = vec![];
        flashed.resize(entries.len(), false);
        Grid {
            entries,
            side_length,
            flashed,
        }
    }

    pub fn increase_energy(&mut self) {
        dbg!(&self.entries);
        let plus_one = u8x32::splat(1);
        let mut chunk_iter = self.entries.chunks_exact_mut(32);
        while let Some(chunk) = chunk_iter.next() {
            let res = u8x32::from_slice(chunk) + plus_one;
            chunk.copy_from_slice(res.as_array())
        }

        // take care of the rest
        for b in chunk_iter.into_remainder() {
            *b += 1;
        }
    }

    pub fn deplete_energy(&mut self) -> u64 {
        let nine = u8x32::splat(9);
        let mut chunk_iter = self.entries.chunks_exact_mut(32);
        let mut flashed_iter = self.flashed.chunks_exact_mut(32);
        let mut flashes = 0;
        let mut chunk_cnt = 0;
        while let Some(chunk) = chunk_iter.next() {
            let flashed_chunk = flashed_iter.next().unwrap();
            let flashed_vec = mask8x32::from_slice(flashed_chunk);
            let res = u8x32::from_slice(chunk).lanes_gt(nine).bitxor(flashed_vec);
            if res.any() {
                // flash
                for l in res.to_array().into_iter().enumerate() {
                    if !l.1 {
                        continue;
                    }

                    let index = chunk_cnt * 32 + l.0;

                    let x = index % self.side_length;
                    let y = index / self.side_length;
                    let surrounding = generate_surrounding_points(self.side_length, x, y);
                    for (px, py) in surrounding.into_iter() {
                        self.inc(px, py);
                    }
                    // get surrounding for l.0
                    // inc
                    // get surrounding
                }
            }

            chunk_cnt += 1;
        }

        // take care of the rest
        for b in chunk_iter.into_remainder() {
            *b += 1;
        }

        flashes
    }

    pub fn inc(&mut self, x: usize, y: usize) {
        let index = y * self.side_length + x;
        self.entries[index] += 1;
    }

    pub fn get_nth_row<'a>(&'a self, n: usize) -> impl Iterator + 'a {
        let offset = n * self.side_length;
        self.entries.iter().skip(offset).take(self.side_length)
    }

    fn get_nth_column_mut<'a>(&'a self, n: usize) -> impl Iterator + 'a {
        let offset = n;
        self.entries.iter().skip(offset).step_by(self.side_length)
    }

    fn get_nth_row_mut<'a>(&'a mut self, n: usize) -> impl Iterator + 'a {
        let offset = n * self.side_length;
        self.entries.iter_mut().skip(offset).take(self.side_length)
    }

    fn get_nth_column<'a>(&'a mut self, n: usize) -> impl Iterator + 'a {
        let offset = n;
        self.entries
            .iter_mut()
            .skip(offset)
            .step_by(self.side_length)
    }
}

fn generate_surrounding_points(side_length: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut points = vec![];
    match (x, y) {
        (0, 0) => {
            points.push((0, 1));
            points.push((1, 1));
            points.push((1, 0));
        }
        (0, vy) => {
            if vy < side_length {
                points.push((0, vy + 1));
                points.push((1, vy + 1));
            }
            points.push((0, vy - 1));
            points.push((1, vy - 1));
            points.push((1, vy));
        }
        (vx, 0) => {
            if vx < side_length {
                points.push((vx + 1, 0));
            }
            points.push((vx - 1, 0));
            points.push((vx, 1));
        }
        (vx, vy) => {
            if vx == side_length && vy == side_length {
                // bottom right corner
                points.push((vx - 1, vy));
                points.push((vx, vy - 1));
                points.push((vx - 1, vy - 1));
            } else {
                points.push((vx - 1, vy));
                points.push((vx, vy - 1));
                points.push((vx - 1, vy - 1));
                points.push((vx + 1, vy + 1));
                if vx < side_length {
                    // right side is fine
                    points.push((vx + 1, vy));
                    points.push((vx + 1, vy - 1));
                }
                if vy < side_length {
                    // bottom is fine
                    points.push((vx, vy + 1));
                    points.push((vx - 1, vy + 1));
                }
            }
        }
    }

    points
}
