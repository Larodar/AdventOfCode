use std::{
    borrow::BorrowMut,
    io::{stdin, BufRead},
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

        let mut new_ranges = vec![];
        for item in &mut seeds.iter_mut() {
            for r in mappings.iter() {
                println!("{:?} with {:?}",&item, r);
                let dst = r.0;
                let lower = r.1;
                let upper = lower + r.2;
                let distance = dst - lower;
                match (
                    (lower..upper).contains(&item.start),
                    (lower..upper).contains(&item.end),
                    (item.start < lower && item.end > upper),
                ) {
                    (_,_, true) => {
                        // |---------------|---------|------|
                        // item.start    lower      upper   item.end
                        let r1 = (lower + distance)..(upper + distance);
                        let r2 = upper..item.end;
                        *item = item.start..lower;
                        println!("1: {:?}, {:?}, {:?}",item, &r1, &r2);
                        new_ranges.push(r1);
                        new_ranges.push(r2);
                        break;
                    }
                    (true, true, _) => {
                        // |---------------|---------|---------|
                        // lower    item.start    item.end  upper
                        *item = (item.start + distance)..(item.end + distance);
                        println!("2: {:?}", item);
                        break;
                    }
                    (true, false, _) => {
                        // |---------------|---------|---------|
                        // lower    item.start    upper     item.end
                        // lower part of seed range is in mapping
                        let r = (item.start + distance)..(upper + distance);
                        *item = upper..item.end;
                        println!("2: {:?}, {:?}", item, &r);
                        new_ranges.push(r);
                        break;
                    }
                    (false, true, _) => {
                        // |---------------|---------|---------|
                        // item.start    lower    item.end    upper
                        // upper part of seed range is in mapping
                        let r = (lower + distance)..(item.end + distance);
                        *item = item.start..lower;
                        println!("2: {:?}, {:?}", item, &r);
                        new_ranges.push(r);
                        break;
                    }
                    (false, false, _) => println!("skip!")
                }
            }
        }

        seeds.retain(|s| !s.is_empty());
        for r in new_ranges.into_iter() {
            assert!(!r.is_empty());
            seeds.push(r);
        }
    }

    seeds.into_iter().map(|r| r.start).min().unwrap() as u64
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
