use std::io::stdin;

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
fn p1(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let grid = PaddedGrid::<DOT>::from_iter(input);
    let mut start = GridPos::new(1, 1);

    let mut total = 0;
    let mut test_count = 0;
    while let Some(p) = grid.position_offset('@' as u8, start) {
        let mut count = 0;
        for p in p.surrounding() {
            if grid.get(p) == Some(b'@') {
                count += 1;
            }
        }

        if count < 4 {
            eprintln!("Candidate at {:?} with {count} surrounding", p);
            total += 1;
        }

        test_count += 1;
        if test_count > 100 {
            panic!();
        }
        if let Some(new_start) = grid.move_one(p) {
            start = new_start;
        } else {
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
        if self.row == usize::MAX || self.col == usize::MAX {
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

    pub fn surrounding(&self) -> SurroundingIter {
        SurroundingIter::new(*self)
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
        pos.row * self.width + pos.col
    }

    fn calc_pos(&self, idx: usize) -> GridPos {
        GridPos::new(idx % self.width, idx / self.width)
    }

    fn move_one(&self, pos: GridPos) -> Option<GridPos> {
        if pos.row == 0 {
            Some(GridPos::new(1, 1))
        } else if pos.col >= self.width - 2 {
            // end of line
            if pos.row >= self.rows - 2 {
                None
            } else {
                Some(GridPos::new(1, pos.row + 1))
            }
        } else {
            Some(GridPos::new(pos.col + 1, pos.row))
        }
    }

    fn get(&self, pos: GridPos) -> Option<u8> {
        if pos.row >= self.rows || pos.col >= self.width {
            return None;
        }

        let idx = self.calc_index(pos);
        if idx >= self.inner.len() {
            return None;
        }

        return Some(self.inner[idx]);
    }

    fn position(&self, needle: u8) -> Option<GridPos> {
        self.inner
            .iter()
            .position(|b| *b == needle)
            .map(|some| self.calc_pos(some))
    }

    fn position_offset(&self, needle: u8, offset: GridPos) -> Option<GridPos> {
        let start = self.calc_index(offset);
        self.inner[start..self.inner.len() - self.width]
            .iter()
            .position(|b| *b == needle)
            .map(|some| self.calc_pos(some + start))
    }

    pub fn points(&self) -> PointIter<'_, PADDING> {
        PointIter {
            grid: self,
            col: 0,
            row: 0,
        }
    }
}

struct PointIter<'a, const PADDING: u8> {
    grid: &'a PaddedGrid<PADDING>,
    col: usize,
    row: usize,
}

impl<'a, const PADDING: u8> PointIter<'a, PADDING> {
    fn new(grid: &'a PaddedGrid<PADDING>) -> PointIter<'a, PADDING> {
        PointIter {
            grid,
            col: 1,
            row: 1,
        }
    }
}

impl<'a, const PADDING: u8> Iterator for PointIter<'a, PADDING> {
    type Item = GridPos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.grid.rows - 2 {
            return None;
        }
        let col = self.col;
        self.col = (self.col + 1) % (self.grid.width - 2);
        let row = self.row;
        self.row = if self.col == 0 { row + 1 } else { row };
        Some(GridPos::new(col, row))
    }
}

struct SurroundingIter {
    pos: GridPos,
    state: u8,
}

impl SurroundingIter {
    fn new(pos: GridPos) -> Self {
        SurroundingIter { pos, state: 8 }
    }
}

impl Iterator for SurroundingIter {
    type Item = GridPos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state == 0 {
            return None;
        }
        let pos = match self.state {
            1 => self.pos.left(),
            2 => self.pos.down_left(),
            3 => self.pos.down(),
            4 => self.pos.down_right(),
            5 => self.pos.right(),
            6 => self.pos.up_right(),
            7 => self.pos.up(),
            8 => self.pos.up_left(),
            _ => unreachable!(),
        };

        self.state -= 1;
        Some(pos)
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
        assert_eq!(Some('a' as u8), grid.get(GridPos::new(1, 1)));
        assert_eq!(Some(DOT), grid.get(GridPos::new(0, 1)));
        assert_eq!(Some(DOT), grid.get(GridPos::new(1, 0)));
        assert_eq!(Some(DOT), grid.get(GridPos::new(0, 0)));
        assert_eq!(Some(DOT), grid.get(GridPos::new(2, 0)));
        assert_eq!(Some(DOT), grid.get(GridPos::new(0, 2)));
        assert_eq!(Some(DOT), grid.get(GridPos::new(1, 2)));
        assert_eq!(Some(DOT), grid.get(GridPos::new(2, 2)));
    }

    #[test]
    fn index_grid_advanced() {
        const A: u8 = 'a' as u8;
        const B: u8 = 'b' as u8;
        const C: u8 = 'c' as u8;
        let input = vec!["aaa", "bbb", "ccc"];
        let grid = PaddedGrid::<DOT>::from_iter(input);
        assert_eq!(Some(A), grid.get(GridPos::new(1, 1)));
        assert_eq!(Some(A), grid.get(GridPos::new(2, 1)));
        assert_eq!(Some(A), grid.get(GridPos::new(3, 1)));
        assert_eq!(Some(B), grid.get(GridPos::new(1, 2)));
        assert_eq!(Some(B), grid.get(GridPos::new(2, 2)));
        assert_eq!(Some(B), grid.get(GridPos::new(3, 2)));
        assert_eq!(Some(C), grid.get(GridPos::new(1, 3)));
        assert_eq!(Some(C), grid.get(GridPos::new(2, 3)));
        assert_eq!(Some(C), grid.get(GridPos::new(3, 3)));
        assert_eq!(Some(DOT), grid.get(GridPos::new(0, 0)));
        assert_eq!(Some(DOT), grid.get(GridPos::new(4, 4)));
        assert_eq!(Some(DOT), grid.get(GridPos::new(4, 0)));
        assert_eq!(Some(DOT), grid.get(GridPos::new(0, 4)));
    }

    #[test]
    fn grid_position() {
        const A: u8 = 'a' as u8;
        let input = vec!["a"];
        let grid = PaddedGrid::<DOT>::from_iter(input);
        assert_eq!(None, grid.position(0u8));
        assert_eq!(Some(GridPos::new(0, 0)), grid.position(DOT));
        assert_eq!(Some(GridPos::new(1, 1)), grid.position(A));
    }

    #[test]
    fn grid_position_offset() {
        const A: u8 = 'a' as u8;
        let input = vec!["a"];
        let grid = PaddedGrid::<DOT>::from_iter(input);
        assert_eq!(None, grid.position_offset(0u8, GridPos::new(0, 0)));
        assert_eq!(
            Some(GridPos::new(0, 0)),
            grid.position_offset(DOT, GridPos::new(0, 0))
        );
        assert_eq!(
            Some(GridPos::new(1, 1)),
            grid.position_offset(A, GridPos::new(0, 0))
        );
    }
    #[test]
    fn grid_position_advanced() {
        let input = vec!["abc", "def", "ghi"];
        let grid = PaddedGrid::<DOT>::from_iter(input);
        assert_eq!(
            Some(GridPos::new(3, 3)),
            grid.position_offset(b'i', GridPos::new(0, 0))
        );
    }

    #[test]
    fn grid_move_one() {
        let input = vec!["abc", "def", "ghi"];
        let grid = PaddedGrid::<DOT>::from_iter(input);
        assert_eq!(Some(GridPos::new(1, 1)), grid.move_one(GridPos::new(0, 0)));
        assert_eq!(Some(GridPos::new(1, 1)), grid.move_one(GridPos::new(1, 0)));
        assert_eq!(Some(GridPos::new(1, 1)), grid.move_one(GridPos::new(2, 0)));
        assert_eq!(Some(GridPos::new(1, 1)), grid.move_one(GridPos::new(0, 1)));
        assert_eq!(Some(GridPos::new(2, 1)), grid.move_one(GridPos::new(1, 1)));
        assert_eq!(Some(GridPos::new(3, 1)), grid.move_one(GridPos::new(2, 1)));
        assert_eq!(Some(GridPos::new(1, 2)), grid.move_one(GridPos::new(0, 2)));
        assert_eq!(Some(GridPos::new(2, 2)), grid.move_one(GridPos::new(1, 2)));
        assert_eq!(Some(GridPos::new(3, 2)), grid.move_one(GridPos::new(2, 2)));
        assert_eq!(Some(GridPos::new(1, 3)), grid.move_one(GridPos::new(0, 3)));
        assert_eq!(Some(GridPos::new(2, 3)), grid.move_one(GridPos::new(1, 3)));
        assert_eq!(Some(GridPos::new(3, 3)), grid.move_one(GridPos::new(2, 3)));
        assert_eq!(None, grid.move_one(GridPos::new(3, 3)));
    }

    #[test]
    fn grid_surround() {
        let input = vec!["abc", "def", "ghi"];
        let grid = PaddedGrid::<DOT>::from_iter(input);
        let p = GridPos::new(2,2);
        assert_eq!(Some(b'a'), grid.get(p.up_left()));
        assert_eq!(Some(b'b'), grid.get(p.up()));
        assert_eq!(Some(b'c'), grid.get(p.up_right()));
        assert_eq!(Some(b'd'), grid.get(p.left()));
        assert_eq!(Some(b'f'), grid.get(p.right()));
        assert_eq!(Some(b'g'), grid.get(p.down_left()));
        assert_eq!(Some(b'h'), grid.get(p.down()));
        assert_eq!(Some(b'i'), grid.get(p.down_right()));
    }

    #[test]
    fn grid_surround_iter() {
        let p = GridPos::new(2,2);
        let mut iter = p.surrounding();
        assert_eq!(Some(GridPos::new(1,1)), iter.next());
        assert_eq!(Some(GridPos::new(2,1)), iter.next());
        assert_eq!(Some(GridPos::new(3,1)), iter.next());
        assert_eq!(Some(GridPos::new(3,2)), iter.next());
        assert_eq!(Some(GridPos::new(3,3)), iter.next());
        assert_eq!(Some(GridPos::new(2,3)), iter.next());
        assert_eq!(Some(GridPos::new(1,3)), iter.next());
        assert_eq!(Some(GridPos::new(1,2)), iter.next());
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
