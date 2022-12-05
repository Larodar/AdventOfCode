use std::io::BufRead;

fn main() {
    match std::env::args()
        .into_iter()
        .skip(1)
        .next()
        .map(|s| s.parse::<u32>().unwrap())
    {
        Some(1) => println!(
            "The Elf with the highest amount of calories has {} calories.",
            find_max_cals(&mut std::io::stdin().lock().lines().map(|r| r.unwrap()))
        ),
        Some(2) => println!(
            "The top three Elfs total amount of calories is {} calories.",
            find_top_three_cals(&mut std::io::stdin().lock().lines().map(|r| r.unwrap()))
        ),
        _ => {}
    }
}

fn find_low(arr: &[u64; 3]) -> Option<(usize, u64)> {
    let mut current = arr[0];
    let mut current_idx = None;

    for i in 1..arr.len() {
        let val = arr[i];
        if val < current {
            current = val;
            current_idx = Some(i);
        }
    }

    if let Some(idx) = current_idx {
        Some((idx, current))
    } else {
        None
    }
}

pub fn find_top_three_cals<I: Iterator<Item = String>>(iter: &mut I) -> u64 {
    let mut top_three = [0u64; 3];
    let mut current_chunk = 0;
    for l in iter {
        if l.is_empty() {
            if let Some(low) = find_low(&top_three) {
                if low.1 < current_chunk {
                    top_three[low.0] = current_chunk;
                }
            }

            current_chunk = 0;
            continue;
        }

        let val: u64 = l.parse().unwrap();
        current_chunk += val;
    }

    if let Some(low) = find_low(&top_three) {
        if low.1 < current_chunk {
            top_three[low.0] = current_chunk;
        }
    }

    top_three.into_iter().sum()
}

pub fn find_max_cals<I: Iterator<Item = String>>(iter: &mut I) -> u64 {
    let mut max = 0;
    let mut current_chunk = 0;
    for l in iter {
        if l.is_empty() {
            if current_chunk > max {
                max = current_chunk
            }

            current_chunk = 0;
            continue;
        }

        let val: u64 = l.parse().unwrap();
        current_chunk += val;
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

        assert_eq!(find_max_cals(&mut input.into_iter()), 24000);
    }

    #[test]
    fn test_p2() {
        let input = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

        let result = find_top_three_cals(&mut input.into_iter());
        assert_eq!(result, 45000);
    }
}
