#![feature(portable_simd)]

use lib_grid_rs::{Grid, GridMut};

const DATA: &[u8] = include_bytes!("../../input");

fn main() {
    match std::env::args().nth(1).map(|s| s.parse::<u32>().unwrap()) {
        Some(1) => println!("{}", p1()),
        Some(2) => println!("{}", p2(DATA)),
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}

pub fn p1() -> u64 {
    let len = DATA.iter().take_while(|b| **b != 0x0A).count();
    let grid = Grid::<1>::new(DATA, len);
    let start = grid.find(0x53).unwrap();
    if let Some(next) = grid.left(start) {
        let count = walk_loop(next, Direction::Left, &grid);
        if count > 0 {
            return (count as f64 / 2.0).ceil() as u64;
        }
    }
    if let Some(next) = grid.up(start) {
        let count = walk_loop(next, Direction::Up, &grid);
        if count > 0 {
            return (count as f64 / 2.0).ceil() as u64;
        }
    }
    if let Some(next) = grid.down(start) {
        let count = walk_loop(next, Direction::Down, &grid);
        if count > 0 {
            return (count as f64 / 2.0).ceil() as u64;
        }
    }
    if let Some(next) = grid.right(start) {
        let count = walk_loop(next, Direction::Right, &grid);
        if count > 0 {
            return (count as f64 / 2.0).ceil() as u64;
        }
    }

    0
}

fn walk_loop(next: (usize, usize), dir: Direction, grid: &Grid<1>) -> u32 {
    let mut w = Walk {
        row: next.1,
        col: next.0,
        direction: dir,
    };
    let mut steps = 0;
    loop {
        if let Some(walk) = try_walk(w, grid) {
            steps += 1;
            w = walk;
        } else {
            steps += 1;
            break;
        }
    }

    steps
}

fn try_walk(current: Walk, grid: &Grid<1>) -> Option<Walk> {
    let seg = grid.get(current.col, current.row).try_into().unwrap();
    match (current.direction, seg) {
        (_, Segment::Start) => {
            // we are through
            None
        }
        (Direction::Up, Segment::Vertical) => Some(Walk {
            row: current.row - 1,
            ..current
        }),
        (Direction::Up, Segment::SouthWest) => Some(Walk {
            col: current.col - 1,
            direction: Direction::Left,
            ..current
        }),
        (Direction::Up, Segment::SouthEast) => Some(Walk {
            col: current.col + 1,
            direction: Direction::Right,
            ..current
        }),
        (Direction::Up, _) => None,
        (Direction::Down, Segment::Vertical) => Some(Walk {
            row: current.row + 1,
            direction: Direction::Down,
            ..current
        }),
        (Direction::Down, Segment::NorthEast) => Some(Walk {
            col: current.col + 1,
            direction: Direction::Right,
            ..current
        }),
        (Direction::Down, Segment::NorthWest) => Some(Walk {
            col: current.col - 1,
            direction: Direction::Left,
            ..current
        }),
        (Direction::Down, _) => None,
        (Direction::Left, Segment::NorthEast) => Some(Walk {
            row: current.row - 1,
            direction: Direction::Up,
            ..current
        }),
        (Direction::Left, Segment::SouthEast) => Some(Walk {
            row: current.row + 1,
            direction: Direction::Down,
            ..current
        }),
        (Direction::Left, Segment::Horizontal) => Some(Walk {
            col: current.col - 1,
            ..current
        }),
        (Direction::Left, _) => None,
        (Direction::Right, Segment::Horizontal) => Some(Walk {
            col: current.col + 1,
            ..current
        }),
        (Direction::Right, Segment::NorthWest) => Some(Walk {
            row: current.row - 1,
            direction: Direction::Up,
            ..current
        }),
        (Direction::Right, Segment::SouthWest) => Some(Walk {
            row: current.row + 1,
            direction: Direction::Down,
            ..current
        }),
        (Direction::Right, _) => None,
    }
}

pub fn p2(data: &[u8]) -> u64 {
    let len = data.iter().take_while(|b| **b != 0x0A).count();
    let grid = Grid::<1>::new(&data[..data.len() - 1], len);

    let mut meta_raw = std::iter::repeat(0)
        .take(data.len() - 1)
        .collect::<Vec<_>>();
    // meta data grid, where the bits of the byte at a position have the meaning
    // 0: -
    // 1: in
    // 2: loop
    // 3:
    // 4: dir1
    // 5: dir2
    // 6:
    // 7:
    let mut meta = GridMut::<1>::new(&mut meta_raw[..], len);
    let start = grid.find(0x53).unwrap();
    if let Some(next) = grid.left(start) {
        let count = walk_loop_p2(next, Direction::Left, &grid, &mut meta);
        if count > 0 {
            return meta.count_and_sum(0x01) as u64;
        }
    }

    if let Some(next) = grid.up(start) {
        let count = walk_loop_p2(next, Direction::Up, &grid, &mut meta);
        if count > 0 {
            return meta.count_and_sum(0x01) as u64;
        }
    }

    if let Some(next) = grid.down(start) {
        let count = walk_loop_p2(next, Direction::Down, &grid, &mut meta);
        if count > 0 {
            return meta.count_and_sum(0x01) as u64;
        }
    }

    if let Some(next) = grid.right(start) {
        let count = walk_loop_p2(next, Direction::Right, &grid, &mut meta);
        if count > 0 {
            return meta.count_and_sum(0x01) as u64;
        }
    }

    0
}

fn walk_loop_p2(
    next: (usize, usize),
    dir: Direction,
    grid: &Grid<1>,
    grid_map: &mut GridMut<1>,
) -> u32 {
    let mut walk = Walk {
        row: next.1,
        col: next.0,
        direction: dir,
    };
    let mut min_row = grid.width();
    let mut min_row_col = 0;

    let mut steps = 0;
    loop {
        if let Some(w) = try_walk(walk, grid) {
            steps += 1;
            let val = 0x02u8 | w.direction as u8;
            grid_map.set(walk.col, walk.row, val);
            if min_row > w.row && w.direction != Direction::Down {
                min_row = walk.row;
                min_row_col = walk.col;
            }

            walk = w;
        } else {
            if steps == 0 {
                return 0;
            }
            let val = 0x02u8 | dir as u8;
            grid_map.set(walk.col, walk.row, val);
            steps += 1;
            break;
        }
    }
    // get orientation at top-most point
    let meta_info = grid_map.get(min_row_col, min_row);
    let mut last_dir: Direction = (meta_info & 0xF0u8).try_into().unwrap();
    let mut c = if last_dir == Direction::Left {
        min_row_col - 1
    } else {
        min_row_col + 1
    };
    let mut r = min_row;
    match last_dir {
        Direction::Left => {
            // We are following the loop to the left.
            // Hence, outside is to the right
            // and inside is to the left
            //
            // Because of turns we may have to check two points per point in the loop.
            while c != min_row_col || r != min_row {
                let meta_info = grid_map.get(c, r);
                let dir: Direction = (meta_info & 0xF0u8).try_into().unwrap();
                let (p1, p2, new_c, new_r) = match (dir, last_dir) {
                    (Direction::Right, Direction::Up) => ((c, r - 1), Some((c - 1, r)), c + 1, r), // mark above and behind as inside
                    (Direction::Right, _) => ((c, r - 1), None, c + 1, r), // mark above as inside
                    (Direction::Left, Direction::Down) => ((c, r + 1), Some((c + 1, r)), c - 1, r), // mark below and behind as inside
                    (Direction::Left, _) => ((c, r + 1), None, c - 1, r), // mark below as inside
                    (Direction::Up, Direction::Right) => ((c - 1, r), Some((c, r - 1)), c, r - 1), // mark left as inside
                    (Direction::Up, _) => ((c - 1, r), None, c, r - 1), // mark left as inside
                    (Direction::Down, Direction::Right) => ((c + 1, r), Some((c, r - 1)), c, r + 1), // mark right and behind as inside
                    (Direction::Down, _) => ((c + 1, r), None, c, r + 1), // mark right as inside
                };
                c = new_c;
                r = new_r;
                last_dir = dir;

                if (grid_map.get(p1.0, p1.1) & 0x02) == 0 {
                    grid_map.set(p1.0, p1.1, 0x01);
                }

                if let Some((col, row)) = p2 {
                    if (grid_map.get(col, row) & 0x02) == 0 {
                        grid_map.set(col, row, 0x01);
                    }
                }
            }
        }
        Direction::Right => {
            // We are following the loop to the right.
            // Hence, outside is to the left
            // and inside is to the right
            while c != min_row_col || r != min_row {
                let meta_info = grid_map.get(c, r);
                let dir: Direction = (meta_info & 0x30u8).try_into().unwrap();
                let (col, row, new_c, new_r) = match dir {
                    Direction::Right => (c, r + 1, c + 1, r), // mark above as inside
                    Direction::Left => (c, r - 1, c - 1, r),  // mark below as inside
                    Direction::Up => (c + 1, r, c, r - 1),    // mark left as inside
                    Direction::Down => (c - 1, r, c, r + 1),  // mark right as inside
                };
                c = new_c;
                r = new_r;

                if (grid_map.get(col, row) & 0x02) == 0 {
                    grid_map.set(col, row, 0x01);
                }
            }
        }
        _ => unreachable!(),
    }

    for r in min_row..grid_map.height() - 1 {
        for c in 1..grid_map.width() - 1 {
            if grid_map.get(c, r) != 0 {
                continue;
            }

            let up = r.saturating_sub(1);
            let left = c.saturating_sub(1);
            let right = c + 1;
            let down = r + 1;
            match (
                grid_map.get(c, up),
                grid_map.get(right, r),
                grid_map.get(c, down),
                grid_map.get(left, r),
            ) {
                (0x01, _, _, _) | (_, 0x01, _, _) | (_, _, 0x01, _) | (_, _, _, 0x01) => {
                    grid_map.set(c, r, 0x01)
                }
                _ => {}
            }
        }
    }

    steps
}
fn print_grid(g: &GridMut<1>) {
    let rows = g.rows();
    for r in rows {
        //let row = r.iter().map(|b| *b + 0x30).collect::<Vec<_>>();
        for b in r.iter().map(|b| *b + 0x30) {
            match b as char {
                'R' => print!("\x1b[93m←\x1b[0m"),
                'b' => print!("\x1b[93m→\x1b[0m"),
                'B' => print!("\x1b[93m↓\x1b[0m"),
                '2' => print!("\x1b[93m↑\x1b[0m"),
                c => print!("{}", c),
            }
        }

        println!()
    }
}

#[derive(Debug, Copy, Clone)]
struct Walk {
    row: usize,
    col: usize,
    direction: Direction,
}

impl AsRef<Walk> for Walk {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up = 0x00,
    Down = 0x10,
    Left = 0x20,
    Right = 0x30,
}

impl TryFrom<u8> for Direction {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Direction::Up),
            0x10 => Ok(Direction::Down),
            0x20 => Ok(Direction::Left),
            0x30 => Ok(Direction::Right),
            _ => Err("Invalid Direction."),
        }
    }
}

/// indicates the kind of pipe segment.
/// | is a vertical pipe connecting north and south.
/// - is a horizontal pipe connecting east and west.
/// L is a 90-degree bend connecting north and east.
/// J is a 90-degree bend connecting north and west.
/// 7 is a 90-degree bend connecting south and west.
/// F is a 90-degree bend connecting south and east.
/// . is ground; there is no pipe in this tile.
/// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
#[repr(u8)]
#[derive(Debug)]
enum Segment {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl TryFrom<u8> for Segment {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x7C => Ok(Segment::Vertical),
            0x2D => Ok(Segment::Horizontal),
            0x4C => Ok(Segment::NorthEast),
            0x4A => Ok(Segment::NorthWest),
            0x37 => Ok(Segment::SouthWest),
            0x46 => Ok(Segment::SouthEast),
            0x2E => Ok(Segment::Ground),
            0x53 => Ok(Segment::Start),
            _ => Err("Not a valid Segment"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2_simple() {
        let input = concat!(
            "..........\n",
            ".S------7.\n",
            ".|F----7|.\n",
            ".||....||.\n",
            ".||....||.\n",
            ".|L-7F-J|.\n",
            ".|..||..|.\n",
            ".L--JL--J.\n",
            "..........\n",
        );

        assert_eq!(p2(input.as_bytes()), 4)
    }

    #[test]
    fn test_bigger() {
        let input = concat!(
            ".F----7F7F7F7F-7....\n",
            ".|F--7||||||||FJ....\n",
            ".||.FJ||||||||L7....\n",
            "FJL7L7LJLJ||LJ.L-7..\n",
            "L--J.L7...LJS7F-7L7.\n",
            "....F-J..F7FJ|L7L7L7\n",
            "....L7.F7||L7|.L7L7|\n",
            ".....|FJLJ|FJ|F7|.LJ\n",
            "....FJL-7.||.||||...\n",
            "....L---J.LJ.LJLJ...\n",
        );

        assert_eq!(p2(input.as_bytes()), 8)
    }

    #[test]
    fn test_p2() {
        let input = concat!(
            "FF7FSF7F7F7F7F7F---7\n",
            "L|LJ||||||||||||F--J\n",
            "FL-7LJLJ||||||LJL-77\n",
            "F--JF--7||LJLJIF7FJ-\n",
            "L---JF-JLJIIIIFJLJJ7\n",
            "|F|F-JF---7IIIL7L|7|\n",
            "|FFJF7L7F-JF7IIL---7\n",
            "7-L-JL7||F7|L7F-7F7|\n",
            "L.L7LFJ|||||FJL7||LJ\n",
            "L7JLJL-JLJLJL--JLJ.L\n",
        );

        assert_eq!(p2(input.as_bytes()), 10)
    }
}
