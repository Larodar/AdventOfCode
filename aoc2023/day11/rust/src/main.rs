#![feature(portable_simd)]

use lib_grid_rs::Grid;
use std::simd::{u8x32, LaneCount, SupportedLaneCount};

const DATA: &[u8] = include_bytes!("../../input");

fn main() {
    match std::env::args().nth(1).map(|s| s.parse::<u32>().unwrap()) {
        Some(1) => println!("{}", p1(&DATA[..DATA.len() - 1])),
        Some(2) => println!("{}", p2()),
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}

pub fn p1(data: &[u8]) -> u64 {
    let grid = Grid::<1>::new(data, data.iter().take_while(|b| **b != 0x0A).count());

    let mut col_count = std::iter::repeat(0).take(grid.width()).collect::<Vec<_>>();
    let mut row_count = std::iter::repeat(0).take(grid.height()).collect::<Vec<_>>();
    let mut points = Vec::with_capacity(128);

    for row in 0..grid.height() {
        for col in 0..grid.width() {
            if grid.get(col, row) == 0x23 {
                col_count[col] += 1;
                row_count[row] += 1;
                points.push((col, row))
            }
        }
    }

    let empty_cols = col_count
        .into_iter()
        .enumerate()
        .filter(|(_, cnt)| *cnt == 0)
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();

    let empty_rows = row_count
        .into_iter()
        .enumerate()
        .filter(|(_, cnt)| *cnt == 0)
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();

    let mut pairs = 0;
    let mut total = 0;
    for idx in 0..points.len() {
        let current = points[idx];
        // walk over the remaining points and calc the distance on the expanded 2d grid.
        for other_idx in idx + 1..points.len() {
            pairs += 1;
            let other = points[other_idx];
            // other is located behind current because of the collection above
            let (exp_col, exp_row) = match other.0.cmp(&current.0) {
                std::cmp::Ordering::Less => (
                    empty_cols
                        .iter()
                        .filter(|i| (current.0..other.0).contains(i))
                        .count(),
                    empty_rows
                        .iter()
                        .filter(|i| (other.1..current.1).contains(i))
                        .count(),
                ),
                std::cmp::Ordering::Equal => (
                    0,
                    empty_rows
                        .iter()
                        .filter(|i| (other.1..current.1).contains(i))
                        .count(),
                ),
                std::cmp::Ordering::Greater => (
                    empty_cols
                        .iter()
                        .filter(|i| (current.0..other.0).contains(i))
                        .count(),
                    empty_rows
                        .iter()
                        .filter(|i| (other.1..current.1).contains(i))
                        .count(),
                ),
            };

            let distance_vec = (
                (other.0 + exp_col) as i64 - current.0 as i64,
                (other.1 + exp_row - current.1) as i64,
            );

            total += distance(distance_vec);
        }
    }

    total as u64
}

// calculate the distance, covered by the given vector in a discrete 2d space.
fn distance((x, y): (i64, i64)) -> usize {
    (x.abs() + y.abs()) as usize
}

pub fn p2() -> u64 {
    0
}

pub fn xor_count_and_sum<const S: usize>(slice: &[u8], xor_op: u8, token: u8) -> usize
where
    LaneCount<S>: SupportedLaneCount,
{
    let preload = slice.len() % S;
    let mut total = slice[..preload].iter().filter(|b| **b == 1).count();

    let xor_vec = u8x32::splat(xor_op);
    let inscribed_vec = u8x32::splat(token);
    for chunk in slice[preload..].chunks_exact(S) {
        let v = u8x32::from_slice(chunk);
        //total += ((xor_vec ^ v) & inscribed_vec).reduce_sum() as usize;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = concat!(
            "...#......\n",
            ".......#..\n",
            "#.........\n",
            "..........\n",
            "......#...\n",
            ".#........\n",
            ".........#\n",
            "..........\n",
            ".......#..\n",
            "#...#.....",
        );

        assert_eq!(p1(input.as_bytes()), 374);
    }

    //#[test]
    //fn test_distance() {
    //    //assert_eq!(distance((1,7)))
    //}
}
