use std::io::Read;

fn main() {
    let input = read_input();
    let res = find_common_ground(&input[..]);
    println!("{} fuel needed to align at {}", res.0, res.1);
}

fn calc_median(input: &mut Vec<u16>) -> u16 {
    input.sort();
    let mid = input.len() / 2;
    input[mid]
}

fn calc_mean(input: &[u16]) -> f64 {
    let sum: u64 = input.iter().fold(0, |v, i| v + (*i as u64));
    sum as f64 / input.len() as f64
}

fn read_input() -> Vec<u16> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn find_common_ground(positions: &[u16]) -> (u64, u16) {
    // use the median here for the first rule set
    let mean = calc_mean(positions).round() as u16;
    let mut current_min = (u64::MAX, 0u16);
    for val in mean - 1..mean + 1 {
        let mut fuel = 0u64;
        for int in positions.iter() {
            let distance = (val as i32 - (*int as i32)).abs();
            let f = (distance.pow(2) + distance) / 2;
            fuel += f as u64;
            // first rule set
            //fuel += distance as u64;
        }

        if current_min.0 > fuel {
            current_min = (fuel, val);
        }
    }

    current_min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![16u16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(find_common_ground(&input[..]), (168, 5));
    }
}
