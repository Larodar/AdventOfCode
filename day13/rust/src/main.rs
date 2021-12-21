use std::fmt::Display;
use std::io::BufRead;
use std::iter::Skip;
use std::iter::StepBy;
use std::iter::Take;
use std::num::ParseIntError;
use std::ops::Add;
use std::str::FromStr;

fn main() {
    let (mut grid, folds) = read_input();
    for fold in folds {
        grid = grid.fold_at(fold);
        let mut cnt = grid.get_points_gt(0).collect::<Vec<_>>();
        cnt.dedup();
    }

    println!("{}", grid);
}

fn read_input() -> (Grid, Vec<Fold>) {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let grid = Grid::from_lines(&mut lines);
    let folds = lines.map(|s| s.unwrap().parse().unwrap()).collect();
    (grid, folds)
}

enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

impl FromStr for Fold {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.trim_start_matches("fold along ");
        let mut parts = val.split('=');
        let orientation = parts.next().unwrap();
        let index = parts.next().unwrap().parse()?;
        let ret = match orientation {
            "x" => Fold::Vertical(index),
            "y" => Fold::Horizontal(index),
            _ => panic!("Invalid fold definition"),
        };
        Ok(ret)
    }
}

#[derive(Debug)]
struct Grid {
    backing: Vec<u8>,
    width: usize,
    length: usize,
}

impl Grid {
    fn fold_at(self, f: Fold) -> Grid {
        let new_points = match f {
            Fold::Horizontal(y) => project_vertical(self.get_points_gt(0), y).collect::<Vec<_>>(),
            Fold::Vertical(x) => project_horizontal(self.get_points_gt(0), x).collect::<Vec<_>>(),
        };

        Grid::from_coordinates(new_points)
    }

    fn get_points_gt(&self, other: u8) -> impl Iterator<Item = Point<usize>> + '_ {
        self.backing
            .iter()
            .enumerate()
            .filter(move |(_, b)| **b > other)
            .map(|(i, _)| self.index_to_point(i))
    }

    fn from_lines(lines: &mut impl Iterator<Item = std::io::Result<String>>) -> Grid {
        let coords = lines
            .map(|r| r.unwrap())
            .take_while(|l| !l.is_empty())
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        Grid::from_coordinates(coords)
    }

    fn index_to_point(&self, index: usize) -> Point<usize> {
        let y = index / self.width;
        let x = index % self.width;
        Point::<_>(x, y)
    }

    fn from_coordinates(coordinates: Vec<Point<usize>>) -> Grid {
        let (x_max, y_max) = coordinates.iter().fold((0, 0), |temp, current| {
            (temp.0.max(current.0), temp.1.max(current.1))
        });
        let width = x_max + 1;
        let length = y_max + 1;

        let mut backing = vec![];
        backing.resize(width * length, 0);
        let mut g = Grid {
            backing,
            width,
            length,
        };

        for p in coordinates {
            g.set_xy(p.0, p.1, 1);
        }

        g
    }

    fn get_surrounding_sparse(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut points = vec![];
        match (x, y) {
            (0, 0) => {
                points.push((0, 1));
                points.push((1, 0));
            }
            (0, vy) => {
                if vy < self.length {
                    points.push((0, vy + 1));
                }
                points.push((0, vy - 1));
                points.push((1, vy));
            }
            (vx, 0) => {
                if vx < self.width {
                    points.push((vx + 1, 0));
                }
                points.push((vx - 1, 0));
                points.push((vx, 1));
            }
            (vx, vy) => {
                if vx == self.width && vy == self.length {
                    points.push((vx - 1, vy));
                    points.push((vx, vy - 1));
                } else {
                    points.push((vx - 1, vy));
                    points.push((vx, vy - 1));
                    if vx < self.width {
                        points.push((vx + 1, vy));
                    }
                    if vy < self.length {
                        points.push((vx, vy + 1));
                    }
                }
            }
        }

        points
    }

    fn get_surrounding_full(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut sparse = self.get_surrounding_sparse(x, y);
        match (x, y) {
            (0, 0) => sparse.push((x + 1, y + 1)),
            (0, y) => {
                sparse.push((x + 1, y - 1));
                if y < self.length {
                    sparse.push((x + 1, y + 1));
                }
            }
            (x, 0) => {
                sparse.push((x - 1, y + 1));
                if x < self.width {
                    sparse.push((x + 1, y + 1));
                }
            }
            (x, y) => {
                // upper left
                sparse.push((x - 1, y - 1));
                // upper right
                sparse.push((x + 1, y - 1));
                if x < self.width && y < self.length {
                    // lower right
                    sparse.push((x + 1, y + 1));
                }
            }
        }

        sparse
    }

    /// x - horizontal
    /// y - vertical
    fn set_xy(&mut self, x: usize, y: usize, val: u8) {
        self.backing[y * self.width + x] = val;
    }

    fn iter_nth_row(&self, n: usize) -> Take<Skip<std::slice::Iter<'_, u8>>> {
        let offset = n * self.length;
        self.backing.iter().skip(offset).take(self.length)
    }

    fn iter_nth_column(&self, n: usize) -> StepBy<Skip<std::slice::Iter<'_, u8>>> {
        let offset = n;
        self.backing.iter().skip(offset).step_by(self.length)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.backing.chunks(self.width) {
            for (i, b) in chunk.iter().enumerate() {
                if i != 0 {
                    write!(f, " ")?;
                }

                write!(f, "{}", b)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn project_vertical<'a, T: Iterator<Item = Point<usize>> + 'a>(
    points: T,
    f: usize,
) -> impl Iterator<Item = Point<usize>> + 'a {
    points.map(move |p| {
        if p.1 > f {
            let diff = p.1 - f;
            if diff > 0 {
                let new_y = f - diff;
                Point(p.0, new_y)
            } else {
                p
            }
        } else {
            p
        }
    })
}

fn project_horizontal<'a, T: Iterator<Item = Point<usize>> + 'a>(
    points: T,
    f: usize,
) -> impl Iterator<Item = Point<usize>> + 'a {
    points.map(move |p| {
        if p.0 > f {
            let diff = p.0 - f;
            if diff > 0 {
                let new_x = f - diff;
                Point(new_x, p.1)
            } else {
                p
            }
        } else {
            p
        }
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point<T>(T, T)
where
    T: Add,
    T: Ord,
    T: FromStr<Err = ParseIntError>;

impl<T> FromStr for Point<T>
where
    T: Add,
    T: Ord,
    T: FromStr<Err = ParseIntError>,
{
    type Err = ParsePointError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(ParsePointError(Some("Invalid points string.".to_string())));
        }

        Ok(Point(parts[0].parse()?, parts[1].parse()?))
    }
}

#[derive(Debug)]
struct ParsePointError(Option<String>);

impl From<ParseIntError> for ParsePointError {
    fn from(v: ParseIntError) -> Self {
        ParsePointError(Some(format!("{}", v)))
    }
}

impl Display for ParsePointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(s) => writeln!(f, "Could not parse point: {}", s),
            None => writeln!(f, "Could not parse point."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![
            Ok("6,10".to_string()),
            Ok("0,14".to_string()),
            Ok("9,10".to_string()),
            Ok("0,3".to_string()),
            Ok("10,4".to_string()),
            Ok("4,11".to_string()),
            Ok("6,0".to_string()),
            Ok("6,12".to_string()),
            Ok("4,1".to_string()),
            Ok("0,13".to_string()),
            Ok("10,12".to_string()),
            Ok("3,4".to_string()),
            Ok("3,0".to_string()),
            Ok("8,4".to_string()),
            Ok("1,10".to_string()),
            Ok("2,14".to_string()),
            Ok("8,10".to_string()),
            Ok("9,0".to_string()),
            Ok("".to_string()),
            Ok("fold along y=7".to_string()),
            Ok("fold along x=5".to_string()),
        ];

        let mut lines = input.into_iter();

        let grid = Grid::from_lines(&mut lines);
        assert_eq!(grid.width, 11);
        assert_eq!(grid.length, 15);
        let folds: Vec<Fold> = lines.map(|s| s.unwrap().parse().unwrap()).collect();
        let mut folds_iter = folds.into_iter();

        let grid = grid.fold_at(folds_iter.next().unwrap());

        println!("{}", &grid);
        let mut cnt = grid.get_points_gt(0).collect::<Vec<_>>();
        cnt.dedup();

        assert_eq!(cnt.len(), 17);

        let grid = grid.fold_at(folds_iter.next().unwrap());

        let mut cnt = grid.get_points_gt(0).collect::<Vec<_>>();
        cnt.dedup();

        assert_eq!(cnt.len(), 16);
    }
}
