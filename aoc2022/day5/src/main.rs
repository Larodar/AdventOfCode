use std::{
    io::{stdin, Read},
    num::ParseIntError,
    str,
    str::FromStr,
};

fn main() {
    let mut buf = String::with_capacity(1024);
    stdin().read_to_string(&mut buf).unwrap();
    match std::env::args()
        .into_iter()
        .skip(1)
        .next()
        .map(|s| s.parse::<u32>().unwrap())
    {
        Some(1) => println!("{}", p1(buf.as_str())),
        Some(2) => println!("{}", p2(buf.as_str())),
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}

fn p1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let empty_line_pos = lines.iter().position(|s| s.is_empty()).unwrap();
    let mut container_positions = vec![];
    for (idx, b) in lines[empty_line_pos - 1].bytes().enumerate() {
        if b != 0x20 {
            container_positions.push(((b - 0x30) as usize, idx));
        }
    }

    let mut cargo: Vec<Vec<u8>> = Vec::with_capacity(container_positions.len());
    for _ in 0..container_positions.len() {
        cargo.push(vec![]);
    }

    for line in &mut lines[..empty_line_pos - 1].iter().rev() {
        let line_raw = line.as_bytes();
        for (stack_number, idx) in &container_positions {
            match line_raw.get(*idx) {
                None | Some(b' ') => continue,
                Some(v) => cargo[stack_number - 1].push(*v),
            }
        }
    }

    for line in &mut lines[empty_line_pos + 1..].iter() {
        let instr: Instruction = line.parse().unwrap();
        for _ in 0..instr.amount {
            let container = cargo[instr.from].pop().unwrap();
            cargo[instr.to].push(container);
        }
    }

    let raw = cargo.iter().map(|v| *v.last().unwrap()).collect::<Vec<_>>();
    String::from_utf8(raw).unwrap()
}

fn p2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let empty_line_pos = lines.iter().position(|s| s.is_empty()).unwrap();
    let mut container_positions = vec![];
    for (idx, b) in lines[empty_line_pos - 1].bytes().enumerate() {
        if b != 0x20 {
            container_positions.push(((b - 0x30) as usize, idx));
        }
    }

    let mut cargo: Vec<Vec<u8>> = Vec::with_capacity(container_positions.len());
    for _ in 0..container_positions.len() {
        cargo.push(vec![]);
    }

    for line in &mut lines[..empty_line_pos - 1].iter().rev() {
        let line_raw = line.as_bytes();
        for (stack_number, idx) in &container_positions {
            match line_raw.get(*idx) {
                None | Some(b' ') => continue,
                Some(v) => cargo[stack_number - 1].push(*v),
            }
        }
    }

    let mut working_stack = vec![];
    for line in &mut lines[empty_line_pos + 1..].iter() {
        let instr: Instruction = line.parse().unwrap();
        for _ in 0..instr.amount {
            working_stack.push(cargo[instr.from].pop().unwrap());
        }

        for _ in 0..instr.amount {
            cargo[instr.to].push(working_stack.pop().unwrap());
        }
    }

    let raw = cargo.iter().map(|v| *v.last().unwrap()).collect::<Vec<_>>();
    String::from_utf8(raw).unwrap()
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');

        let amount = parts.nth(1).unwrap().parse()?;
        let from: usize = parts.nth(1).unwrap().parse()?;
        let to: usize = parts.nth(1).unwrap().parse()?;
        Ok(Instruction {
            amount,
            from: (from - 1),
            to: (to - 1),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = r"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!("CMZ", p1(input));
    }

    #[test]
    fn p2_test() {
        let input = r"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!("MCD", p2(input));
    }

    #[test]
    fn parse_instruction() {
        let input = "move 1 from 2 to 1";
        assert_eq!(
            Instruction {
                amount: 1,
                from: 1,
                to: 0
            },
            input.parse().unwrap()
        );
    }
}
