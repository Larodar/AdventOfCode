fn main() {
    let input_str =
        std::fs::read_to_string("/home/larodar/source/AdventOfCode/day1/input_2.txt").unwrap();
    let input = input_str
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<i32>>();
    let res = get_inc_windowed(input);
    println!("{}", res);
}

fn get_inc_windowed(input: Vec<i32>) -> i32 {
    let mut increasing = 0;
    let mut last = 0;
    for ptr in 0..(input.len() - 2) {
        let current = &input[ptr..ptr + 3];
        let current_sum = current.iter().sum();
        if last == 0 {
            last = current_sum;
            continue;
        } else if last < current_sum {
            increasing += 1;
        }
        last = current_sum
    }
    increasing
}

fn get_increasing_measurements(input: Vec<i32>) -> i32 {
    let mut iter = input.into_iter();
    let mut increasing = 0;
    let mut current = iter.next().unwrap();
    while let Some(entry) = iter.next() {
        if current < entry {
            increasing += 1;
        }
        current = entry;
    }

    increasing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let items = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(get_increasing_measurements(items), 7)
    }

    #[test]
    fn test_windowed() {
        let items = vec![607, 618, 618, 617, 647, 716, 769, 792];
        assert_eq!(get_inc_windowed(items), 5);
    }
}
