use std::{
    io::{BufRead, Read, stdin},
    ops::Deref,
};

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
fn p1(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    const DOT: u8 = '.' as u8;
    let first = input.next().unwrap();
    let width = first.as_ref().as_bytes().len() + 2;
    let mut buf: Vec<u8> = std::iter::repeat_n(DOT, width).collect();
    buf.push(DOT);
    buf.extend_from_slice(first.as_ref().as_bytes());
    buf.push(DOT);
    while let Some(l) = input.next() {
        buf.push(DOT);
        buf.extend_from_slice(l.as_ref().as_bytes());
        buf.push(DOT);
    }

    for b in std::iter::repeat_n(DOT, width) {
        buf.push(b);
    }

    let grid = Grid::new(&buf[..], width);
    let mut start = GridPos::new(1, 1);

    let mut total = 0;
    let mut test_count = 0;
    while let Some(p) = grid.position_offset('@' as u8, start) {
        let count = grid.get(p.up()).map_or(0, |p| if p == DOT { 0 } else { 1 })
            + grid
                .get(p.up_right())
                .map_or(0, |p| if p == DOT { 0 } else { 1 })
            + grid
                .get(p.right())
                .map_or(0, |p| if p == DOT { 0 } else { 1 })
            + grid
                .get(p.down_right())
                .map_or(0, |p| if p == DOT { 0 } else { 1 })
            + grid
                .get(p.down())
                .map_or(0, |p| if p == DOT { 0 } else { 1 })
            + grid
                .get(p.down_left())
                .map_or(0, |p| if p == DOT { 0 } else { 1 })
            + grid
                .get(p.left())
                .map_or(0, |p| if p == DOT { 0 } else { 1 })
            + grid
                .get(p.up_left())
                .map_or(0, |p| if p == DOT { 0 } else { 1 });
        if count < 4 {
            total += 1;
        }

        test_count += 1;
        if let Some(new_start) = grid.move_one(p) {
            start = dbg!(new_start);
        } else {
            break;
        }
        if test_count > 6 {
            break;
        }
    }

    total
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct GridPos {
    col: usize,
    row: usize,
}

impl GridPos {
    fn new(col: usize, row: usize) -> GridPos {
        GridPos { col, row }
    }

    fn down(&self) -> GridPos {
        if self.row == usize::MAX {
            panic!();
        }

        return GridPos {
            col: self.col,
            row: self.row + 1,
        };
    }

    fn down_left(&self) -> GridPos {
        if self.row == usize::MAX || self.col == 0 {
            panic!();
        }

        GridPos {
            col: self.col - 1,
            row: self.row + 1,
        }
    }

    fn down_right(&self) -> GridPos {
        if self.row == 0 || self.col == usize::MAX {
            panic!();
        }

        GridPos {
            col: self.col + 1,
            row: self.row + 1,
        }
    }

    fn up(&self) -> GridPos {
        if self.row == 0 {
            panic!();
        }

        GridPos {
            col: self.col,
            row: self.row - 1,
        }
    }

    fn up_left(&self) -> GridPos {
        if self.row == 0 || self.col == 0 {
            panic!();
        }

        GridPos {
            col: self.col - 1,
            row: self.row - 1,
        }
    }

    fn up_right(&self) -> GridPos {
        if self.row == 0 || self.col == usize::MAX {
            panic!();
        }

        GridPos {
            col: self.col + 1,
            row: self.row - 1,
        }
    }

    fn right(&self) -> GridPos {
        if self.col == usize::MAX {
            panic!();
        }

        GridPos {
            col: self.col + 1,
            row: self.row,
        }
    }

    fn left(&self) -> GridPos {
        if self.col == 0 {
            panic!();
        }

        GridPos {
            col: self.col - 1,
            row: self.row,
        }
    }
}

struct GridPtr(usize);
impl Deref for GridPtr {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct Grid<'a> {
    inner: &'a [u8],
    width: usize,
    rows: usize,
}

impl<'a> Grid<'a> {
    fn new(data: &'a [u8], width: usize) -> Grid<'a> {
        Grid {
            inner: data,
            width,
            rows: data.len() / width,
        }
    }

    fn move_one(&self, pos: GridPos) -> Option<GridPos> {
        // assumes padding
        if pos.col == self.width - 2 {
            if pos.row == self.rows {
                return None;
            } else {
                return Some(GridPos::new(1, pos.row + 1));
            }
        } else {
            return Some(GridPos::new(pos.col + 1, pos.row));
        }
    }

    fn get(&self, pos: GridPos) -> Option<u8> {
        let idx = pos.row * self.width + pos.col;
        if idx >= self.inner.len() {
            return None;
        }

        return Some(self.inner[idx]);
    }

    fn position(&self, needle: u8) -> Option<GridPos> {
        self.inner
            .iter()
            .position(|b| *b == needle)
            .map(|some| GridPos {
                row: some / self.width,
                col: some % self.width,
            })
    }

    fn position_offset(&self, needle: u8, offset: GridPos) -> Option<GridPos> {
        let start = dbg!(offset.col * self.width + offset.row);
        self.inner[start..]
            .iter()
            .position(|b| *b == needle)
            .map(|some| GridPos {
                row: (some + start) / self.width,
                col: (some + start) % self.width,
            })
    }
}

fn p2(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1_reference() {
        let input = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ];

        assert_eq!(13, p1(input.iter()));
    }
}
