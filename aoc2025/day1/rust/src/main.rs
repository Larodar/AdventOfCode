use std::io::{BufRead, stdin};

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
fn p1(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let mut dial: i64 = 50;
    let mut zeros = 0;
    for l in input.filter(|l| !l.as_ref().is_empty()) {
        let dir = l.as_ref().as_bytes()[0];
        let count: i64 = l.as_ref()[1..].parse().unwrap();
        if dir == ('R' as u8) {
            dial = (dial + count) % 100;
        } else {
            dial = (dial - count) % 100;
        }

        if dial == 0 {
            zeros += 1;
        }
    }

    zeros
}
fn p2(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let mut dial: i64 = 50;
    let mut zero_passes = 0;
    for l in input.filter(|l| !l.as_ref().is_empty()) {
        let dir = l.as_ref().as_bytes()[0];
        let count: i64 = l.as_ref()[1..].parse().unwrap();
        if dir == ('R' as u8) {
            let sum = dial + count;
            zero_passes += sum / 100;
            dial = sum % 100;
        } else {
            let diff = dial - count;
            if diff < 0 {
                let offset = if dial == 0 { 0 } else { 1 };
                zero_passes += (diff / -100) + offset;
                dial = (100 - (diff.abs() % 100)) % 100;
            } else {
                if diff == 0 {
                    zero_passes += 1;
                }
                dial = diff;
            }
        }
   }

    zero_passes as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1_reference() {
        let input = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
        assert_eq!(3, p1(input.lines()));
    }

    #[test]
    fn p2_reference() {
        let input = r#"
L68
L30
R48
L5
R60"#;
        assert_eq!(3, p2(input.lines()));

        let input = r#"
L68
L30
R48
L5
R60
L55"#;
        assert_eq!(4, p2(input.lines()));

        let input = r#"
L68
L30
R48
L5
R60
L55
L1"#;
        assert_eq!(4, p2(input.lines()));

        let input = r#"
L68
L30
R48
L5
R60
L55
L1
L99"#;
        assert_eq!(5, p2(input.lines()));

        let input = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14"#;
        assert_eq!(5, p2(input.lines()));

        let input = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
        assert_eq!(6, p2(input.lines()));
    }

    #[test]
    fn p2_reference_large_step_left() {
        let input = r#"
L300
L300"#;
        assert_eq!(6, p2(input.lines()));
    }

    #[test]
    fn p2_reference_large_step_right() {
        let input = r#"
R300
R300"#;
        assert_eq!(6, p2(input.lines()));
    }

    #[test]
    fn p2_reference_exact_zero_1() {
        let input = r#"
R50
R50
L50"#;
        assert_eq!(2, p2(input.lines()));
    }

    #[test]
    fn p2_reference_exact_zero_2() {
        let input = r#"
R50
R50
L50
L50"#;
        assert_eq!(2, p2(input.lines()));
    }

    #[test]
    fn p2_reference_exact_zero_3() {
        let input = r#"
L50
L199"#;
        assert_eq!(2, p2(input.lines()));
    }

    #[test]
    fn p2_reference_exact_zero_4() {
        let input = r#"
L50
L200"#;
        assert_eq!(3, p2(input.lines()));
    }

    #[test]
    fn p2_reference_exact_zero_5() {
        let input = r#"
L50
L5"#;
        assert_eq!(1, p2(input.lines()));
    }

    #[test]
    fn p2_reference_exact_zero_6() {
        let input = r#"
L50
L5
R60"#;
        assert_eq!(2, p2(input.lines()));
    }

    #[test]
    fn p2_reference_large_right() {
        let input = r#"
R1000"#;
        assert_eq!(10, p2(input.lines()));
    }

    #[test]
    fn p2_reference_large_right_1() {
        let input = r#"
44
R56"#;
        assert_eq!(1, p2(input.lines()));

        let input = r#"
44
R156"#;
        assert_eq!(2, p2(input.lines()));
    }

    #[test]
    fn p2_reference_large_right_2() {
        let input = r#"
44
R556"#;
        assert_eq!(6, p2(input.lines()));
    }

    #[test]
    fn p2_reference_large_left() {
        let input = r#"
R41
L329"#;
        assert_eq!(3, p2(input.lines()));
    }

    #[test]
    fn p2_reference_large_left_2() {
        let input = r#"
R4
L254
R1"#;
        assert_eq!(3, p2(input.lines()));
    }
}
