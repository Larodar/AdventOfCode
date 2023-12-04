use std::io::{stdin, BufRead};

pub const DOT: u8 = 0x2E;
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
    let mut total = 0;
    let pad_once = &[DOT];

    let mut mid = input.next().unwrap();
    let mut bot = input.next().unwrap();
    let mut dot_line = std::iter::repeat(&DOT).take(mid.as_ref().as_bytes().len() + 2);

    total += process_lines(
        &mut dot_line.clone(),
        &mut pad_once
            .iter()
            .chain(mid.as_ref().as_bytes())
            .chain(pad_once.iter()),
        &mut pad_once
            .iter()
            .chain(bot.as_ref().as_bytes())
            .chain(pad_once.iter()),
        mid.as_ref().as_bytes(),
    );

    let mut top = mid;
    mid = bot;
    while let Some(new) = input.next() {
        bot = new;
        total += process_lines(
            &mut pad_once
                .iter()
                .chain(top.as_ref().as_bytes())
                .chain(pad_once.iter()),
            &mut pad_once
                .iter()
                .chain(mid.as_ref().as_bytes())
                .chain(pad_once.iter()),
            &mut pad_once
                .iter()
                .chain(bot.as_ref().as_bytes())
                .chain(pad_once.iter()),
            mid.as_ref().as_bytes(),
        );
        top = mid;
        mid = bot;
    }

    // do one more iter with padding
    total += process_lines(
        &mut pad_once
            .iter()
            .chain(top.as_ref().as_bytes())
            .chain(pad_once.iter()),
        &mut pad_once
            .iter()
            .chain(mid.as_ref().as_bytes())
            .chain(pad_once.iter()),
        &mut dot_line,
        mid.as_ref().as_bytes(),
    );

    total
}

#[derive(Debug, PartialEq)]
enum ReadState {
    None,
    SymbolMid,
    PartNumber,
    SymbolTopBot,
    Digit,
}

fn process_lines<
    'a,
    I1: Iterator<Item = &'a u8>,
    I3: Iterator<Item = &'a u8>,
    I2: Iterator<Item = &'a u8>,
>(
    top: &mut I1,
    mid: &mut I2,
    bot: &mut I3,
    data: &[u8],
) -> u64 {
    let iter = top
        .zip(mid.zip(bot))
        .skip(1)
        .map(|v| (*v.0, *v.1 .0, *v.1 .1));

    let mut total = 0;
    let mut state = ReadState::None;
    let mut num_start = 0;
    for (idx, slice) in iter.enumerate() {
        state = match (state, check(slice)) {
            (ReadState::None, ReadState::Digit) => {
                num_start = idx;
                ReadState::Digit
            }
            (ReadState::None, ReadState::PartNumber) => {
                num_start = idx;
                ReadState::PartNumber
            }
            (ReadState::None, s) => s,
            (ReadState::Digit, s) if s == ReadState::SymbolTopBot || s == ReadState::SymbolMid => {
                // build number
                total += std::str::from_utf8(&data[num_start..idx])
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                s
            }
            (ReadState::Digit, s) => s,
            (ReadState::PartNumber, ReadState::PartNumber | ReadState::Digit) => {
                ReadState::PartNumber
            }
            (ReadState::PartNumber, s) => {
                // build number
                total += std::str::from_utf8(&data[num_start..idx])
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                s
            }
            (
                ReadState::SymbolMid | ReadState::SymbolTopBot,
                ReadState::Digit | ReadState::PartNumber,
            ) => {
                num_start = idx;
                ReadState::PartNumber
            }
            (_, s) => s,
        }
    }

    total
}

fn check(sl: (u8, u8, u8)) -> ReadState {
    match (sl.0, sl.1, sl.2) {
        // above and below are not a symbol
        (DOT | 0x30..=0x39, DOT, DOT | 0x30..=0x39) => ReadState::None,
        (DOT | 0x30..=0x39, 0x30..=0x39, DOT | 0x30..=0x39) => ReadState::Digit,
        // symbol above or below
        (_, DOT, _) => ReadState::SymbolTopBot,
        (_, 0x30..=0x39, _) => ReadState::PartNumber,
        (_, _, _) => ReadState::SymbolMid,
    }
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
        assert_eq!(
            p1(input.into_iter()),
            688 + 95 + 728 + 896 + 153
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
        let input = vec!["357.123", "...*..."];

        assert_eq!(p1(input.into_iter()), 480);
    }
}
