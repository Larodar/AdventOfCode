use std::fmt::Display;
use std::io::BufRead;
use std::iter::Skip;
use std::iter::StepBy;
use std::iter::Take;
use std::num::ParseIntError;
use std::ops::Add;
use std::str::FromStr;
fn main() {
    let (grid, folds) = read_input();
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

struct Grid {
    backing: Vec<u8>,
    width: usize,
    length: usize,
}

impl Grid {
    fn from_lines<'a>(lines: &mut impl Iterator<Item = std::io::Result<String>>) -> Grid {
        let coords = lines
            .map(|r| r.unwrap())
            .take_while(|l| !l.is_empty())
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        Grid::from_coordinates(coords)
    }

    fn from_coordinates(coordinates: Vec<Point<usize>>) -> Grid {
        let (x_max, y_max) = coordinates.iter().fold((0, 0), |temp, current| {
            (temp.0.max(current.0), temp.1.max(current.1))
        });
        let mut backing = vec![];
        backing.resize(x_max * y_max, 0);
        // write coordinates into vec
        Grid {
            backing,
            width: x_max,
            length: y_max,
        }
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
        self.backing[y * self.length + x] = val;
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
