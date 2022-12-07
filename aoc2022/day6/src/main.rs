#![feature(portable_simd)]

use std::{
    io::{stdin, Read},
    simd::{u8x16, u8x4, Mask, SimdUint},
};

fn main() {
    let mut buf = Vec::new();
    _ = stdin().read_to_end(&mut buf).unwrap();

    match std::env::args()
        .into_iter()
        .skip(1)
        .next()
        .map(|s| s.parse::<u32>().unwrap())
    {
        Some(1) => println!("{}", p1(&buf[..])),
        Some(2) => println!("{}", p2(&buf[..])),
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}

fn p1(input: &[u8]) -> u64 {
    for start in 0..input.len() - 3 {
        let end = start + 3;
        let check = u8x16::from_array([
            0xFF,
            input[start],
            input[start],
            input[start],
            input[start + 1],
            0xFF,
            input[start + 1],
            input[start + 1],
            input[start + 2],
            input[start + 2],
            0xFF,
            input[start + 2],
            input[start + 3],
            input[start + 3],
            input[start + 3],
            0xFF,
        ]);

        let window_vec = u8x16::from_array([
            input[start],
            input[start + 1],
            input[start + 2],
            input[start + 3],
            input[start],
            input[start + 1],
            input[start + 2],
            input[start + 3],
            input[start],
            input[start + 1],
            input[start + 2],
            input[start + 3],
            input[start],
            input[start + 1],
            input[start + 2],
            input[start + 3],
        ]);

        if (check ^ window_vec).reduce_min() != 0 {
            return (end + 1) as u64;
        }
    }

    panic!("no start sequence found");
}

fn p2(input: &[u8]) -> u64 {
    let mut window = u8x16::from_slice(&input[..16]);
    window[14] = 0xFF;
    window[15] = 0xFF;

    for i in 14..input.len() {
        let mut found = false;
        for idx in 0..14 {
            let b = window[idx];
            let mut check = u8x16::splat(b);
            check[idx] = 0xFF;
            if (check ^ window).reduce_min() == 0 {
                found = true;
                break;
            }
        }

        if found {
            window = window.rotate_lanes_left::<1>();
            window[13] = input[i];
            window[15] = 0xFF;
        } else {
            return i as u64;
        }
    }

    panic!("no message marker found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(7, p1(input.as_bytes()));

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(5, p1(input.as_bytes()));

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(6, p1(input.as_bytes()));

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(10, p1(input.as_bytes()));

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(11, p1(input.as_bytes()));
    }

    #[test]
    fn test_p2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(19, p2(input.as_bytes()));
    }

    #[test]
    fn test_p2_2() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(26, p2(input.as_bytes()));
    }
    #[test]
    fn test_p2_3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(29, p2(input.as_bytes()));
    }
    #[test]
    fn test_p2_4() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(23, p2(input.as_bytes()));
    }
    #[test]
    fn test_p2_5() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(23, p2(input.as_bytes()));
    }
}
