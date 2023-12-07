use std::{
    borrow::BorrowMut,
    io::{stdin, BufRead},
    ops::Range,
};

fn main() {
    match std::env::args().nth(1).map(|s| s.parse::<u32>().unwrap()) {
        Some(1) => println!(
            "{}",
            p1(stdin()
                .lock()
                .lines()
                .map(|line_result| line_result.unwrap()))
        ),
        Some(2) => println!(
            "{}",
            p2(stdin()
                .lock()
                .lines()
                .map(|line_result| line_result.unwrap()))
        ),
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}

fn p1(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let mut seeds = input
        .next()
        .unwrap()
        .as_ref()
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    while let Some(l) = input
        .borrow_mut()
        .skip_while(|l| l.as_ref().is_empty())
        .next()
    {
        assert!(l.as_ref().ends_with(':'));
        let mappings = input
            .borrow_mut()
            .take_while(|l| !l.as_ref().is_empty())
            .map(|l| {
                let mut iter = l
                    .as_ref()
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.trim().parse::<i64>().unwrap());
                let dst = iter.next().unwrap();
                let src = iter.next().unwrap();
                let len = iter.next().unwrap();
                (dst, src, len)
            })
            .collect::<Vec<_>>();

        for item in seeds.iter_mut() {
            for r in mappings.iter() {
                let dst = r.0;
                let lower = r.1;
                let upper = lower + r.2;
                if (lower..upper).contains(item) {
                    *item += dst - lower;
                    break;
                }
            }
        }
    }

    seeds.into_iter().min().unwrap() as u64
}

fn p2(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let seeds = input
        .next()
        .unwrap()
        .as_ref()
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut seeds = seeds
        .windows(2)
        .step_by(2)
        .map(|pair| pair[0]..(pair[0] + pair[1]))
        .collect::<Vec<_>>();

    let mut transformed = vec![];
    while let Some(l) = input.find(|l| !l.as_ref().is_empty()) {
        assert!(l.as_ref().ends_with(':'));
        let mappings = input
            .borrow_mut()
            .take_while(|l| !l.as_ref().is_empty())
            .map(|l| {
                let mut iter = l
                    .as_ref()
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.trim().parse::<i64>().unwrap());
                let dst = iter.next().unwrap();
                let src = iter.next().unwrap();
                let len = iter.next().unwrap();
                (dst, src, len)
            })
            .collect::<Vec<_>>();

        transform(&mut seeds, &mappings[..], &mut transformed);

        seeds.retain(|s| !s.is_empty());
        while let Some(r) = transformed.pop() {
            assert!(!r.is_empty());
            seeds.push(r);
        }
    }

    seeds.into_iter().map(|r| dbg!(r.start)).min().unwrap() as u64
}

fn transform(
    seeds: &mut Vec<Range<i64>>,
    mappings: &[(i64, i64, i64)],
    transformed: &mut Vec<Range<i64>>,
) {
    let mut idx = 0;
    loop {
        if idx >= seeds.len() {
            break;
        }

        let item = seeds[idx].clone();
        for r in mappings.iter() {
            let dst = r.0;
            let lower = r.1;
            let upper = lower + r.2;
            let distance = dst - lower;
            match (
                (lower..upper).contains(&item.start),
                (lower..upper).contains(&item.end),
                (item.start < lower && item.end > upper),
            ) {
                (false, false, true) => {
                    // |---------------|---------|------|
                    // item.start    lower      upper   item.end
                    transformed.push((lower + distance)..(upper + distance));
                    seeds.push(upper..item.end);
                    seeds[idx].end = lower;
                    break;
                }
                (true, true, _) => {
                    // |---------------|---------|---------|
                    // lower    item.start    item.end  upper
                    seeds[idx] = (item.start + distance)..(item.end + distance);
                    break;
                }
                (true, false, _) => {
                    // |---------------|---------|---------|
                    // lower    item.start    upper     item.end
                    // lower part of seed range is in mapping
                    transformed.push((item.start + distance)..(upper + distance));
                    seeds[idx].start = upper;
                    break;
                }
                (false, true, _) => {
                    // |---------------|---------|---------|
                    // item.start    lower    item.end    upper
                    // upper part of seed range is in mapping
                    transformed.push((lower + distance)..(item.end + distance));
                    seeds[idx].end = lower;
                    break;
                }
                (false, false, _) => {}
            }
        }
        idx += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];

        assert_eq!(p1(input.into_iter()), 35);
    }

    #[test]
    fn sample_p2() {
        let input = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];

        assert_eq!(p2(input.into_iter()), 46);
    }
}
