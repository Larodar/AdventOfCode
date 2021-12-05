use std::collections::HashMap;
use std::fmt::Display;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let lines = read_input();
    let count = get_covered_points(lines);
    println!("{}", count);
}

fn get_covered_points(lines: Vec<Line>) -> usize {
    let mut coverage: HashMap<Point, u16> = HashMap::new();

    for line in lines {
        for p in line.points() {
            if let std::collections::hash_map::Entry::Vacant(e) = coverage.entry(p) {
                e.insert(1);
            } else {
                *coverage.get_mut(&p).unwrap() += 1;
            }
        }
    }

    coverage.into_iter().filter(|(_, cnt)| *cnt >= 2).count()
}

fn read_input() -> Vec<Line> {
    let mut lines = vec![];
    for l in std::io::stdin().lock().lines() {
        let line = l.unwrap().parse().unwrap();
        lines.push(line);
    }

    lines
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(u16, u16);

impl FromStr for Point {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line(Point, Point);

impl Line {
    pub fn points(&self) -> PointsIter {
        PointsIter {
            start: self.0,
            end: self.1,
            exhausted: false,
        }
    }
}

struct PointsIter {
    start: Point,
    end: Point,
    exhausted: bool,
}

impl Iterator for PointsIter {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        let current = self.start;
        if current == self.end {
            self.exhausted = true;
        } else {
            let x = current.0 as i32 - self.end.0 as i32;
            let x = match x.cmp(&0) {
                std::cmp::Ordering::Less => 1,
                std::cmp::Ordering::Greater => -1,
                std::cmp::Ordering::Equal => x,
            };
            let y = current.1 as i32 - self.end.1 as i32;
            let y = match y.cmp(&0) {
                std::cmp::Ordering::Less => 1,
                std::cmp::Ordering::Greater => -1,
                std::cmp::Ordering::Equal => y,
            };
            self.start = Point((current.0 as i32 + x) as u16, (current.1 as i32 + y) as u16);
        }

        Some(current)
    }
}

impl FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("->").map(|v| v.trim()).collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(ParseLineError(Some("Invalid points string.".to_string())));
        }

        Ok(Line(parts[0].parse()?, parts[1].parse()?))
    }
}

#[derive(Debug)]
struct ParseLineError(Option<String>);

impl From<ParsePointError> for ParseLineError {
    fn from(v: ParsePointError) -> Self {
        ParseLineError(Some(format!("Could not parse the line: {}", v)))
    }
}

impl Display for ParseLineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(s) => writeln!(f, "Could not parse line: {}", s),
            None => writeln!(f, "Could not parse line."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_points() {
        let input = vec![
            Line(Point(0, 9), Point(5, 9)),
            Line(Point(9, 4), Point(3, 4)),
            Line(Point(2, 2), Point(2, 1)),
            Line(Point(7, 0), Point(7, 4)),
            Line(Point(0, 9), Point(2, 9)),
            Line(Point(3, 4), Point(1, 4)),
        ];

        let count = get_covered_points(input);
        assert_eq!(count, 5);
    }

    #[test]
    fn parse_points() {
        let input = "0,9";
        let res: Point = input.parse().unwrap();
        assert_eq!(res, Point(0, 9));

        let input = "8,2";
        let res: Point = input.parse().unwrap();
        assert_eq!(res, Point(8, 2));
    }

    #[test]
    fn parse_lines() {
        let input = "0,9 -> 5,9";
        let res: Line = input.parse().unwrap();
        assert_eq!(res, Line(Point(0, 9), Point(5, 9)));

        let input = "5,5 -> 8,2";
        let res: Line = input.parse().unwrap();
        assert_eq!(res, Line(Point(5, 5), Point(8, 2)));
    }

    #[test]
    fn points_iter() {
        let line = Line(Point(1, 1), Point(3, 3));
        let res = line.points().collect::<Vec<_>>();
        assert_eq!(res.len(), 3);

        assert_eq!(res[0], Point(1, 1));
        assert_eq!(res[1], Point(2, 2));
        assert_eq!(res[2], Point(3, 3));

        let line = Line(Point(9, 7), Point(7, 9));
        let res = line.points().collect::<Vec<_>>();
        assert_eq!(res.len(), 3);

        assert_eq!(res[0], Point(9, 7));
        assert_eq!(res[1], Point(8, 8));
        assert_eq!(res[2], Point(7, 9));

        let line = Line(Point(5, 5), Point(8, 2));
        let res = line.points().collect::<Vec<_>>();
        assert_eq!(res.len(), 4);

        assert_eq!(res[0], Point(5, 5));
        assert_eq!(res[1], Point(6, 4));
        assert_eq!(res[2], Point(7, 3));
        assert_eq!(res[3], Point(8, 2));
    }
}
