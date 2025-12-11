use std::{io::stdin, ops::Range};

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
    let mut ranges = vec![];

    while let Some(l) = input.next() {
        let l = l.as_ref();
        if l.is_empty() {
            break;
        }

        let (lower, upper) = l.split_once(|c| c == '-').unwrap();
        let lower: u64 = lower.parse().unwrap();
        let upper: u64 = upper.parse().unwrap();
        ranges.push(lower..upper + 1);
    }

    let mut total = 0;
    while let Some(l) = input.next() {
        let l = l.as_ref();
        if l.is_empty() {
            break;
        }

        let val: u64 = l.parse().unwrap();
        if ranges.iter().any(|r| r.contains(&val)) {
            total += 1;
        }
    }

    total
}

fn p2(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let mut ranges: Vec<Range<u64>> = vec![];

    while let Some(l) = input.next() {
        let l = l.as_ref();
        if l.is_empty() {
            break;
        }

        let (lower, upper) = l.split_once(|c| c == '-').unwrap();
        let lower: u64 = lower.parse().unwrap();
        let upper = upper.parse::<u64>().unwrap() + 1;
        let current = lower..upper;
        insert_range(&mut ranges, current);
    }

    ranges.retain(|r| !r.is_empty());
    consolidate_ranges(&mut ranges);
    ranges.sort_unstable_by_key(|r| r.start);

    ranges.retain(|r| !r.is_empty());
    ranges.iter().map(|r| (r.end - r.start) as u64).sum()
}

fn consolidate_ranges(ranges: &mut Vec<Range<u64>>) {
    let mut merged = true;
    while merged {
        merged = false;
        ranges.retain(|r| !r.is_empty());
        for i in 0..ranges.len() {
            for lower in 0..i {
                if ranges[lower].is_empty() {
                    continue;
                }

                if let Some(res1) = merge_ranges(&ranges[i], &ranges[lower]) {
                    // increase the current
                    ranges[i] = res1;
                    // set the range that was merged to empty
                    ranges[lower] = 0..0;
                    merged = true;
                }
            }

            for upper in i + 1..ranges.len() {
                if ranges[upper].is_empty() {
                    continue;
                }

                if let Some(res1) = merge_ranges(&ranges[i], &ranges[upper]) {
                    // increase the current
                    ranges[i] = res1;
                    // set the range that was merged to empty
                    ranges[upper] = 0..0;
                    merged = true;
                }
            }
        }
    }
}

fn insert_range(ranges: &mut Vec<Range<u64>>, r: Range<u64>) {
    for i in 0..ranges.len() {
        let local = &ranges[i];
        if let Some(res1) = merge_ranges(&local, &r) {
            ranges[i] = res1;
            return;
        }
    }

    ranges.push(r);
}

fn merge_ranges(rhs: &Range<u64>, lhs: &Range<u64>) -> Option<Range<u64>> {
    match (rhs.contains(&lhs.start), rhs.contains(&lhs.end)) {
        (true, true) => {
            // lhs is subset of rhs
            Some(rhs.clone())
        }
        (true, false) => {
            // append lhs to rhs
            Some(rhs.start..lhs.end)
        }
        (false, true) => {
            // prepend lhs to rhs
            Some(lhs.start..rhs.end)
        }
        (false, false) => {
            // distinct ranges
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1_reference() {
        let input = vec![
            "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
        ];

        assert_eq!(3, p1(input.iter()));
    }

    #[test]
    fn p2_reference() {
        let input = vec![
            "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
        ];

        assert_eq!(14, p2(input.iter()));
    }

    #[test]
    fn p2_reference2() {
        let input = vec![
            "10-14", "16-20", "12-18", "3-5", "", "1", "5", "8", "11", "17", "32",
        ];

        assert_eq!(14, p2(input.iter()));
    }

    #[test]
    fn p2_reference3() {
        let input = vec!["2-4", "5-7", "8-10"];

        assert_eq!(9, p2(input.iter()));
    }

    #[test]
    fn p2_reference4() {
        let input = vec!["2-4", "5-7", "8-10", "11-11"];

        assert_eq!(10, p2(input.iter()));
    }

    #[test]
    fn consolidate_test_1() {
        let mut input = vec![10..19, 16..21];
        consolidate_ranges(&mut input);
        assert_eq!(input.len(), 1);
        assert_eq!(input[0].start, 10);
        assert_eq!(input[0].end, 21);
    }

    #[test]
    fn consolidate_test_2() {
        let mut input = vec![
            222235372532517..229028822916591,
            225742115230077..227522359650711,
        ];
        consolidate_ranges(&mut input);
        assert_eq!(input.len(), 1);
        assert_eq!(input[0].start, 222235372532517);
        assert_eq!(input[0].end, 229028822916591);
    }
}
