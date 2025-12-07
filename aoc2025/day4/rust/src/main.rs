use std::{io::stdin, ops::Deref};

const DOT: u8 = '.' as u8;
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
fn p1(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let grid = PaddedGrid::<DOT>::from_iter(input);
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

#[derive(Debug, Copy, Clone)]
struct GridPtr(usize);
impl Deref for GridPtr {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct PaddedGrid<const PADDING: u8> {
    inner: Vec<u8>,
    width: usize,
    rows: usize,
}

impl<'a, const PADDING: u8, L: AsRef<str>> FromIterator<L> for PaddedGrid<PADDING> {
    fn from_iter<T: IntoIterator<Item = L>>(iter: T) -> Self {
        let mut i = iter.into_iter();
        let first = i.next().unwrap();
        let width = first.as_ref().as_bytes().len() + 2;
        let mut buf: Vec<u8> = std::iter::repeat_n(PADDING, width).collect();
        buf.push(PADDING);
        buf.extend_from_slice(first.as_ref().as_bytes());
        buf.push(PADDING);
        while let Some(l) = i.next() {
            buf.push(PADDING);
            buf.extend_from_slice(l.as_ref().as_bytes());
            buf.push(PADDING);
        }

        for b in std::iter::repeat_n(PADDING, width) {
            buf.push(b);
        }

        PaddedGrid::new(buf, width)
    }
}

impl<const PADDING: u8> PaddedGrid<PADDING> {
    fn new(data: Vec<u8>, width: usize) -> PaddedGrid<PADDING> {
        let rows = data.len() / width;
        PaddedGrid {
            inner: data,
            width,
            rows,
        }
    }

    fn calc_index(&self, pos: GridPos) -> usize {
        self.width + pos.row * self.width + pos.col + 1
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
        if pos.row + 2 >= self.rows || pos.col + 2 >= self.width {
            return None;
        }

        let idx = self.calc_index(pos);
        if idx >= self.inner.len() {
            return None;
        }

        return Some(self.inner[idx]);
    }

    fn position(&self, needle: u8) -> Option<GridPos> {
        if needle == PADDING {
            return None;
        }

        self.inner.iter().position(|b| *b == needle).map(|some| {
            let row = (some / self.width) - 1;
            let col = (some % self.width) - 1;
            GridPos { row, col }
        })
    }

    fn position_offset(&self, needle: u8, offset: GridPos) -> Option<GridPos> {
        if needle == PADDING {
            return None;
        }

        let start = self.calc_index(offset);
        self.inner[start..self.inner.len() - self.width]
            .iter()
            .position(|b| *b == needle)
            .map(|some| {
                let row = ((some + start) / self.width) - 1;
                let col = ((some + start) % self.width) - 1;
                GridPos { row, col }
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
    fn construct_grid() {
        let input = "a";
        let grid = PaddedGrid::<DOT>::from_iter(input.lines());
        assert_eq!(9, grid.inner.len());
        assert_eq!('a' as u8, grid.inner[4]);
        assert_eq!(DOT, grid.inner[0]);
        assert_eq!(DOT, grid.inner[1]);
        assert_eq!(DOT, grid.inner[2]);
        assert_eq!(DOT, grid.inner[3]);
        assert_eq!(DOT, grid.inner[5]);
        assert_eq!(DOT, grid.inner[6]);
        assert_eq!(DOT, grid.inner[7]);
        assert_eq!(DOT, grid.inner[8]);
    }

    #[test]
    fn index_grid() {
        let input = "a";
        let grid = PaddedGrid::<DOT>::from_iter(input.lines());
        assert_eq!(Some('a' as u8), grid.get(GridPos::new(0, 0)));
        assert_eq!(None, grid.get(GridPos::new(0, 1)));
        assert_eq!(None, grid.get(GridPos::new(1, 0)));
        assert_eq!(None, grid.get(GridPos::new(1, 1)));
    }

    #[test]
    fn index_grid_advanced() {
        const A: u8 = 'a' as u8;
        const B: u8 = 'b' as u8;
        const C: u8 = 'c' as u8;
        let input = vec!["aaa", "bbb", "ccc"];
        let grid = PaddedGrid::<DOT>::from_iter(input);
        assert_eq!(Some(A), grid.get(GridPos::new(0, 0)));
        assert_eq!(Some(A), grid.get(GridPos::new(1, 0)));
        assert_eq!(Some(A), grid.get(GridPos::new(2, 0)));
        assert_eq!(Some(B), grid.get(GridPos::new(0, 1)));
        assert_eq!(Some(B), grid.get(GridPos::new(1, 1)));
        assert_eq!(Some(B), grid.get(GridPos::new(2, 1)));
        assert_eq!(Some(C), grid.get(GridPos::new(0, 2)));
        assert_eq!(Some(C), grid.get(GridPos::new(1, 2)));
        assert_eq!(Some(C), grid.get(GridPos::new(2, 2)));
        assert_eq!(None, grid.get(GridPos::new(0, 3)));
        assert_eq!(None, grid.get(GridPos::new(3, 2)));
    }

    #[test]
    fn grid_position() {
        const A: u8 = 'a' as u8;
        let input = vec!["a"];
        let grid = PaddedGrid::<DOT>::from_iter(input);
        assert_eq!(None, grid.position(0u8));
        assert_eq!(None, grid.position(DOT));
        assert_eq!(Some(GridPos::new(0, 0)), grid.position(A));
    }

    #[test]
    fn grid_position_offset() {
        const A: u8 = 'a' as u8;
        let input = vec!["a"];
        let grid = PaddedGrid::<DOT>::from_iter(input);
        assert_eq!(None, grid.position_offset(0u8, GridPos::new(0, 0)));
        assert_eq!(None, grid.position_offset(DOT, GridPos::new(0, 0)));
        assert_eq!(
            Some(GridPos::new(0, 0)),
            grid.position_offset(A, GridPos::new(0, 0))
        );
    }

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
