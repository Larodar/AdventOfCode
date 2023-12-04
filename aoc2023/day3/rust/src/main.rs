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

#[derive(Debug, PartialEq)]
enum ReadState {
    None,
    Symbol,
    PartNumber,
    Digit,
}

fn p1(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let mut last_line = input.next().unwrap();
    // mask tells us what numbers are still to evaluate and the symbols of the last line.
    let mut carry = (0..last_line.as_ref().as_bytes().len())
        .map(|_| 0x2E)
        .collect::<Vec<_>>();
    carry.fill(0x2E);
    let mut carry_next = carry.clone();
    carry_next.fill(0x2E);
    let mut total = process_line(last_line.as_ref(), &mut carry[..], &mut carry_next[..]);
    for current in input {
        assert_eq!(last_line.as_ref().len(), current.as_ref().len());
        total += process_line(current.as_ref(), &mut carry[..], &mut carry_next[..]);
        last_line = current;
    }

    total
}

fn process_line(line: &str, carry: &mut [u8], next_carry: &mut [u8]) -> u64 {
    let mut total = 0;
    let mut state = ReadState::None;
    let mut num_start = 0;
    let line_bytes = line.as_bytes();
    assert_eq!(line_bytes.len(), carry.len());
    assert_eq!(line_bytes.len(), next_carry.len());

    for idx in 0..line_bytes.len() {
        state = match (state, line_bytes[idx], carry[idx]) {
            //  .
            // ..
            (ReadState::None, 0x2E, 0x2E | 0x30..=0x39) => ReadState::None,
            (ReadState::None, 0x2E, _) => ReadState::Symbol,
            //  .
            // .1
            (ReadState::None, 0x30..=0x39, 0x2E | 0x30..=0x39) => {
                num_start = idx;
                ReadState::Digit
            }
            (ReadState::None, 0x30..=0x39, _) => {
                num_start = idx;
                ReadState::PartNumber
            }
            //  1
            // .$
            (ReadState::None, s, _) => {
                next_carry[idx] = s;
                ReadState::Symbol
            }
            (ReadState::Digit, 0x2E, 0x2E | 0x30..=0x39) => {
                next_carry[num_start..idx].copy_from_slice(&line_bytes[num_start..idx]);
                ReadState::None
            }
            (ReadState::Digit, 0x2E, _) => {
                next_carry[num_start..idx].copy_from_slice(&line_bytes[num_start..idx]);
                total += std::str::from_utf8(&line_bytes[num_start..idx])
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                ReadState::Symbol
            }
            (ReadState::Digit, 0x30..=0x39, 0x2E | 0x30..=0x39) => ReadState::Digit,
            (ReadState::Digit, 0x30..=0x39, _) => ReadState::PartNumber,
            (ReadState::Digit, s, _) => {
                next_carry[idx] = s;
                total += std::str::from_utf8(&line_bytes[num_start..idx])
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                ReadState::Symbol
            }
            (ReadState::Symbol, 0x2E, _) => ReadState::None,
            (ReadState::Symbol, 0x30..=0x39, _) => {
                num_start = idx;
                ReadState::PartNumber
            }
            (ReadState::Symbol, s, _) => {
                next_carry[idx] = s;
                ReadState::Symbol
            }
            (ReadState::PartNumber, 0x30..=0x39, _) => ReadState::PartNumber,
            (ReadState::PartNumber, 0x2E, _) => {
                total += std::str::from_utf8(&line_bytes[num_start..idx])
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                ReadState::None
            }
            (ReadState::PartNumber, s, _) => {
                next_carry[idx] = s;
                ReadState::Symbol
            }
        }
    }

    if state == ReadState::Digit {
        next_carry[num_start..].copy_from_slice(&line_bytes[num_start..]);
    }

    let mut i = 0;
    loop {
        if i >= next_carry.len() {
            break;
        }

        let val = next_carry[i];
        if val != 0x2E && !(0x30..=0x39).contains(&val) {
            // hit a symbol
            let before = i.saturating_sub(1);
            let after = std::cmp::min(i + 1, carry.len());
            let expr = ((0x30..=0x39).contains(&carry[before]),
                (0x30..=0x39).contains(&carry[i]),
                (0x30..=0x39).contains(&carry[after]));
            match expr {
                (true, false, true) => {
                    let start = before
                        - carry[..before]
                            .iter()
                            .rev()
                            .take_while(|b| (0x30..=0x39).contains(*b))
                            .count();
                    total += std::str::from_utf8(&carry[start..i])
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();

                    let end = after
                        + carry[after..]
                            .iter()
                            .take_while(|b| (0x30..=0x39).contains(*b))
                            .count();
                    total += std::str::from_utf8(&carry[after..end])
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                }
                (true, true, true) => {
                    let start = before
                        - carry[..before]
                            .iter()
                            .rev()
                            .take_while(|b| (0x30..=0x39).contains(*b))
                            .count();
                    let end = after
                        + carry[after..]
                            .iter()
                            .take_while(|b| (0x30..=0x39).contains(*b))
                            .count();
                    total += std::str::from_utf8(&carry[start..end])
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                }
                (true, set, _) => {
                    let start = before
                        - carry[..before]
                            .iter()
                            .rev()
                            .take_while(|b| (0x30..=0x39).contains(*b))
                            .count();
                    let end = if set { i + 1} else {i};
                    total += std::str::from_utf8(&carry[start..end])
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                }
                (_, true, _) => {
                    let end = i + carry[i..]
                        .iter()
                        .take_while(|b| (0x30..=0x39).contains(*b))
                        .count();
                    total += std::str::from_utf8(&carry[i..end])
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                }
                (_, _, true) => {
                    let end = after
                        + carry[after..]
                            .iter()
                            .take_while(|b| (0x30..=0x39).contains(*b))
                            .count();
                    total += std::str::from_utf8(&carry[after..end])
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                    i = end;
                }
                (_, _, _) => {}
            }
        }
        i += 1;
    }

    carry.copy_from_slice(next_carry);
    next_carry.fill(0x2E);
    total
}

fn p2(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    //let mut scnd_last = input.next().unwrap();
    //let mut last = input.next().unwrap();
    //if let Some(pos) = scnd_last.as_ref().as_bytes().iter().position(|b| *b == 0x2A) {
    //    let adj_nums = 0;
    //}
    //if let Some(pos) = last.as_ref().as_bytes().iter().position(|b| *b == 0x2A) {
    //    
    //}
    //while let Some(line) = input.next(){
    //    let current = line.as_ref();



    //    scnd_last = last;
    //    last = line;
    //}

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        assert_eq!(p1(input.into_iter()), 4361);
    }

    #[test]
    fn error_case() {
        let input = vec![
"..............423....688..934............970................................95.728..........896...113..................153..972.............",
"...122..................*.....*..........................919..509*..........&...@.........../...........................+.......*...........",
"....+..........259....698..373.992.52.674.........................781...22........130.584.....-...%399.......777.................266........",
"......148..+....*........................*.....357.123.......................812.........*756.143...........*...............................",
"..691*.....700..708................-...357........*........$177......%..244.............................762.453....477-.707..-168..359*.....",
".......................394.443....456......750..................71.160.....*..183.........835..74.........*.............../............129..",
        ];
        assert_eq!(p1(input.into_iter()), 688 + 95 + 728 + 896 + 153
            // line 2
            + 122 + 509
            // line 3
            + 259 + 698 +373 + 992 + 674 + 781 + 584 + 399 + 777 + 266
            // line 4
            + 148 + 357 + 123 + 756 + 143
            // line 5
            + 691 + 700 + 708 + 357 + 177 + 244 + 453 + 762 + 707 + 477 + 168 + 359
            // line 6
            + 456 + 160 + 129
            );
    }

    #[test]
    fn two_numbers() {
        let input = vec!["..357.123..", ".....*....."];

        assert_eq!(p1(input.into_iter()), 480);
    }

    #[test]
    fn edges() {
        let input = vec![
            "357.123",
            "...*..."
        ];

        assert_eq!(p1(input.into_iter()), 480);
    }
}
