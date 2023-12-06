use std::io::{stdin, BufRead};

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

pub fn p1(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let l1 = input.next().unwrap();
    let times = l1
        .as_ref()
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap());
    let l2 = input.next().unwrap();
    let distances = l2
        .as_ref()
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap());

    let mut total = 1;
    for (time, dist) in times.zip(distances) {
        let mut window_size = 0;
        for held in 1..time {
            if held * (time - held) > dist {
                window_size += 1;
            }
        }

        total *= window_size;
    }

    total
}

pub fn p2(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let time = input
        .next()
        .unwrap()
        .as_ref()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let distance = input
        .next()
        .unwrap()
        .as_ref()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    // acceleration = 1
    //
    // distance = speed * (time - charge_time)
    // speed = charge_time since acceleration is 1
    // distance = charge_time * (time - charge_time)
    // distance = charge_time * time - charge_time ^ 2
    // 0 = charge_time * time - charge_time ^ 2 - distance
    // midnight formula
    // c1/c2 = (-time +/- sqrt(time ^ 2 - (4 * distance / acceleration)) / -2 * acceleration
    // since acceleration is 1
    // c1/c2 = (-time +/- sqrt(time ^ 2 - (4 * distance)) / -2
    let sqrt = ((time.pow(2) - (4 * distance)) as f64).sqrt();

    let time_f = -time as f64;
    let lower = (time_f + sqrt) / (-2f64);
    let upper = (time_f - sqrt) / (-2f64);

    let lower = lower.ceil() as u64;
    let upper = upper.floor() as u64;

    upper - lower + 1 // we have to include both bounds
}
