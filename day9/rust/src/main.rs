use std::io::BufRead;

fn main() {
    let field = read_std_in();
    let mins = search_mins(&field);
    dbg!(&mins);
    let result: u64 = mins.into_iter().map(|p| (p.2 + 1) as u64).sum();
    println!("{}", result);
}

fn read_std_in() -> Field {
    let mut lines = vec![];
    while let Some(line) = std::io::stdin().lock().lines().next() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }

        lines.push(l.bytes().map(|b| b - 48).collect());
    }

    lines.into()
}

fn search_mins(f: &Field) -> Vec<(usize, usize, u8)> {
    let mut ret = vec![];
    for x in 0..f.area.len() {
        for y in 0..f.area[0].len() {
            if is_local_min(f, x, y) {
                ret.push((x, y, f.get_value(x, y).unwrap()));
            }
        }
    }

    ret
}

fn is_local_min(f: &Field, x: usize, y: usize) -> bool {
    let current = f.get_value(x, y).unwrap();
    generate_surrounding_points(f, x, y)
        .into_iter()
        .map(|(temp_x, temp_y)| (temp_x, temp_y, f.get_value(temp_x, temp_y).unwrap()))
        .find(|v| v.2 < current)
        .is_none()
}

#[derive(Debug, Default)]
struct Field {
    area: Vec<Vec<u8>>,
    width: usize,
    length: usize,
}

impl Field {
    pub fn new(area: Vec<Vec<u8>>) -> Field {
        let width = area[0].len() - 1;
        let length = area.len() - 1;
        Field {
            area,
            width,
            length,
        }
    }

    pub fn get_value(&self, x: usize, y: usize) -> Option<u8> {
        if x > self.length || y > self.width {
            dbg!(self);
            println!("({}, {})", x, y);
            None
        } else {
            Some(self.area[x][y])
        }
    }
}

impl From<Vec<Vec<u8>>> for Field {
    fn from(v: Vec<Vec<u8>>) -> Self {
        Field::new(v)
    }
}

fn generate_surrounding_points(f: &Field, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut points = vec![];
    match (x, y) {
        (0, 0) => {
            points.push((0, 1));
            points.push((1, 0));
            points.push((1, 1));
        }
        (0, vy) => {
            if vy < f.width {
                points.push((1, vy + 1));
                points.push((0, vy + 1));
            }
            points.push((0, vy - 1));
            points.push((1, vy - 1));
            points.push((1, vy));
        }
        (vx, 0) => {
            if vx < f.length {
                points.push((vx + 1, 0));
                points.push((vx + 1, 1));
            }
            points.push((vx - 1, 0));
            points.push((vx - 1, 1));
            points.push((vx, 1));
        }
        (vx, vy) => {
            if vx == f.length && vy == f.width {
                points.push((vx - 1, vy - 1));
                points.push((vx - 1, vy));
                points.push((vx, vy - 1));
            } else {
                points.push((vx - 1, vy - 1));
                points.push((vx - 1, vy));
                points.push((vx, vy - 1));
                if vx < f.length {
                    points.push((vx + 1, vy - 1));
                    points.push((vx + 1, vy));
                    if vy < f.width {
                        points.push((vx + 1, vy + 1));
                    }
                }
                if vy < f.width {
                    points.push((vx - 1, vy + 1));
                    points.push((vx, vy + 1));
                }
            }
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_min() {
        let input = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ]
        .into_iter()
        .map(|s| s.bytes().map(|b| b - 48).collect::<Vec<u8>>())
        .collect::<Vec<_>>();
        let field = input.into();

        assert!(is_local_min(&field, 0, 1));
        assert!(is_local_min(&field, 2, 2));
        assert!(!is_local_min(&field, 4, 3));
        assert!(is_local_min(&field, 0, 9));
    }

    #[test]
    fn points_iter() {
        let input = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ]
        .into_iter()
        .map(|s| s.bytes().map(|b| b - 48).collect::<Vec<u8>>())
        .collect::<Vec<_>>();
        let field = input.into();

        let points = generate_surrounding_points(&field, 0, 0);
        assert_eq!(points.len(), 3);
        assert!(points.contains(&(0, 1)));
        assert!(points.contains(&(1, 1)));
        assert!(points.contains(&(1, 0)));

        let points = generate_surrounding_points(&field, 0, 3);
        assert_eq!(points.len(), 5);
        assert!(points.contains(&(0, 4)));
        assert!(points.contains(&(0, 2)));
        assert!(points.contains(&(1, 2)));
        assert!(points.contains(&(1, 3)));
        assert!(points.contains(&(1, 4)));

        let points = generate_surrounding_points(&field, 2, 0);
        assert_eq!(points.len(), 5);
        assert!(points.contains(&(1, 0)));
        assert!(points.contains(&(1, 1)));
        assert!(points.contains(&(2, 1)));
        assert!(points.contains(&(3, 0)));
        assert!(points.contains(&(3, 1)));

        let points = generate_surrounding_points(&field, 1, 1);
        assert_eq!(points.len(), 8);
        assert!(points.contains(&(0, 0)));
        assert!(points.contains(&(0, 1)));
        assert!(points.contains(&(0, 2)));
        assert!(points.contains(&(1, 0)));
        assert!(points.contains(&(1, 2)));
        assert!(points.contains(&(2, 0)));
        assert!(points.contains(&(2, 1)));
        assert!(points.contains(&(2, 2)));

        let points = generate_surrounding_points(&field, field.length, field.width);
        assert_eq!(points.len(), 3);
        assert!(points.contains(&(field.length, field.width - 1)));
        assert!(points.contains(&(field.length - 1, field.width)));
        assert!(points.contains(&(field.length - 1, field.width - 1)));

        let points = generate_surrounding_points(&field, field.length, 0);
        dbg!(&points);
        assert_eq!(points.len(), 3);
        assert!(points.contains(&(field.length, 1)));
        assert!(points.contains(&(field.length - 1, 0)));
        assert!(points.contains(&(field.length - 1, 1)));
    }
}
