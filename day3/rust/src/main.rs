fn main() {
    let input_str =
        std::fs::read_to_string("/home/v1p3r/source/AdventOfCode/day3/input.txt").unwrap();
    // part 1
    //let (counts, total) = collect_bits(&input_str);
    //let (gamma, epsilon) = calculate_rates(counts, total);
    //println!("{}", gamma * epsilon);
    let items = input_str
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let o2_gen_rating = find_oxygen_gen_rating(&items[..]);
    let co2_scrubber_rating = find_co2_scrubber_rating(&items[..]);
    println!("{}", o2_gen_rating * co2_scrubber_rating);
}

fn find_oxygen_gen_rating(input: &[&str]) -> u64 {
    let oxy_gen = filter_for_bit_at_pos(input, 0, find_common_bit_in_column);
    bit_string_to_u64(oxy_gen)
}

fn find_co2_scrubber_rating(input: &[&str]) -> u64 {
    let co2_scrub = filter_for_bit_at_pos(input, 0, find_uncommon_bit_in_column);
    bit_string_to_u64(co2_scrub)
}

fn bit_string_to_u64<T: AsRef<str>>(s: T) -> u64 {
    if s.as_ref().len() > 64 {
        panic!("String is too long.");
    }

    let mut ret = 0;
    for (i, b) in s.as_ref().bytes().rev().enumerate() {
        let bit = match b {
            0x30 => 0,
            0x31 => 1,
            _ => panic!("Invalid data."),
        };
        ret |= bit << i;
    }

    ret
}

fn filter_for_bit_at_pos<F>(items: &[&str], pos: usize, find_bit_fn: F) -> String
where
    F: Fn(&[&str], usize) -> u8,
{
    let bit = find_bit_fn(items, pos);
    let remaining = items
        .iter()
        .filter(|s| s.bytes().nth(pos).unwrap() == bit)
        .map(|s| *s)
        .collect::<Vec<_>>();
    if remaining.len() == 1 {
        remaining[0].to_string()
    } else {
        filter_for_bit_at_pos(&remaining[..], pos + 1, find_bit_fn)
    }
}

fn find_uncommon_bit_in_column(items: &[&str], pos: usize) -> u8 {
    let res = check_column_bit_proportion(items, pos);
    if res < 0 {
        0x31
    } else {
        0x30
    }
}

fn check_column_bit_proportion(items: &[&str], pos: usize) -> i32 {
    let mut res = 0;
    items
        .iter()
        .map(|s| match s.bytes().nth(pos).unwrap() {
            0x30 => res -= 1,
            0x31 => res += 1,
            _ => panic!("Ahhhhh!"),
        })
        .reduce(|_, _| {});
    res
}

fn find_common_bit_in_column(items: &[&str], pos: usize) -> u8 {
    let res = check_column_bit_proportion(items, pos);
    if res >= 0 {
        0x31
    } else {
        0x30
    }
}

fn collect_bits(input: &str) -> (Vec<u64>, usize) {
    let len = input.lines().nth(1).unwrap().len();
    let mut collect = Vec::with_capacity(len);
    collect.resize(len, 0u64);

    let entries = input
        .split('\n')
        .map(|s| {
            if s.is_empty() {
                return;
            }
            for (i, b) in s.as_bytes().iter().enumerate() {
                collect[i] += (*b - 0x30) as u64;
            }
        })
        .count();

    (collect, entries)
}

fn calculate_rates(counts: Vec<u64>, total: usize) -> (u64, u64) {
    let mut gamma = 0u64;
    let width = counts.len();
    for (i, v) in counts.into_iter().enumerate() {
        let bit: u64 = if v * 2 < total as u64 { 0 } else { 1 };
        gamma |= (bit << (width - 1 - i)) as u64;
    }

    let mut mask = 0u64;
    for i in 0..width {
        mask |= 1 << i;
    }

    (gamma, (!gamma & mask))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_text() {
        let input = concat!(
            "00100\n", "11110\n", "10110\n", "10111\n", "10101\n", "01111\n", "00111\n", "11100\n",
            "10000\n", "11001\n", "00010\n", "01010"
        );

        let (res, count) = collect_bits(input);
        assert_eq!(count, 12);
        assert_eq!(res, [7, 5, 8, 7, 5]);
    }

    #[test]
    fn calc_gamma() {
        let counts = vec![7u64, 5, 8, 7, 5];
        let total = 12;
        let (gamma, epsilon) = calculate_rates(counts, total);
        assert_eq!(gamma, 0b10110);
        assert_eq!(epsilon, 0b01001);
        assert_eq!(gamma * epsilon, 198);
    }

    #[test]
    fn find_oxy_gen() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let oxy_gen = filter_for_bit_at_pos(&input[..], 0, find_common_bit_in_column);
        assert_eq!(oxy_gen, "10111");

        assert_eq!(find_oxygen_gen_rating(&input[..]), 23);
    }

    #[test]
    fn find_co2_scrub() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let co2_scrub = filter_for_bit_at_pos(&input[..], 0, find_uncommon_bit_in_column);
        assert_eq!(co2_scrub, "01010");

        assert_eq!(find_co2_scrubber_rating(&input[..]), 10);
    }

    #[test]
    fn find_uncommon_bit() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let bit = find_uncommon_bit_in_column(&input[..], 0);
        assert_eq!(bit, 0x30);

        let bit = find_uncommon_bit_in_column(&input[..], 1);
        assert_eq!(bit, 0x31);

        let bit = find_uncommon_bit_in_column(&input[..], 2);
        assert_eq!(bit, 0x30);

        let bit = find_uncommon_bit_in_column(&input[..], 3);
        assert_eq!(bit, 0x30);

        let bit = find_uncommon_bit_in_column(&input[..], 4);
        assert_eq!(bit, 0x31);
    }
}
