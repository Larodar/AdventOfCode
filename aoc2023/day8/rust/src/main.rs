use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    str::FromStr,
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
    let instructions = input.next().unwrap();

    let mut nodes: HashMap<[u8; 3], Node> = HashMap::with_capacity(1024);
    for node in input
        .skip_while(|l| l.as_ref().is_empty())
        .map(|l| l.as_ref().parse::<Node>().unwrap())
    {
        _ = nodes.insert(node.name, node);
    }

    walk(
        [0x41, 0x41, 0x41],
        instructions.as_ref().as_bytes(),
        0,
        &nodes,
        0,
    )
    .try_into()
    .unwrap()
}

fn walk(
    current_name: [u8; 3],
    dirs: &[u8],
    mut dir_ptr: usize,
    nodes: &HashMap<[u8; 3], Node>,
    steps: i64,
) -> i64 {
    if current_name == [0x5A, 0x5A, 0x5A] {
        return steps;
    }

    if dir_ptr >= dirs.len() {
        dir_ptr = 0;
    }

    let current = nodes.get(&current_name).unwrap();
    match dirs[dir_ptr].into() {
        Direction::Left => walk(current.left, dirs, dir_ptr + 1, nodes, steps + 1),
        Direction::Right => walk(current.right, dirs, dir_ptr + 1, nodes, steps + 1),
    }
}

enum Direction {
    Left,
    Right,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0x4C => Direction::Left,
            0x52 => Direction::Right,
            _ => panic!(),
        }
    }
}

struct Node {
    name: [u8; 3],
    left: [u8; 3],
    right: [u8; 3],
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        Ok(Node {
            name: bytes[..3].try_into().unwrap(),
            left: bytes[7..10].try_into().unwrap(),
            right: bytes[12..15].try_into().unwrap(),
        })
    }
}

fn p2(mut input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let instructions = input.next().unwrap();

    let mut nodes: HashMap<[u8; 3], Node> = HashMap::with_capacity(1024);
    let mut start = Vec::with_capacity(16);
    for node in input
        .skip_while(|l| l.as_ref().is_empty())
        .map(|l| l.as_ref().parse::<Node>().unwrap())
    {
        if node.name[2] == 0x41 {
            start.push(dbg!(node.name));
        }
        _ = nodes.insert(node.name, node);
    }

    walk_many(&mut start[..], instructions.as_ref().as_bytes(), &nodes)
}

fn walk_many(current_names: &mut [[u8; 3]], dirs: &[u8], nodes: &HashMap<[u8; 3], Node>) -> u64 {
    let mut dir_ptr = 0;
    let mut steps = 0;
    while !dbg!(&current_names).iter().all(|name| name[2] == 0x5A) {
        match dirs[dir_ptr].into() {
            Direction::Left => {
                for name in current_names.iter_mut() {
                    *name = nodes.get(name.as_ref()).unwrap().left;
                }
            }
            Direction::Right => {
                for name in current_names.iter_mut() {
                    *name = nodes.get(name.as_ref()).unwrap().right;
                }
            }
        }

        dir_ptr = if dir_ptr == dirs.len() - 1 {
            0
        } else {
            dir_ptr + 1
        };
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = vec![
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ];

        assert_eq!(2, p1(input.into_iter()))
    }

    #[test]
    fn sample_repeat() {
        let input = vec![
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ];

        assert_eq!(6, p1(input.into_iter()))
    }

    #[test]
    fn sample_p2() {
        let input = vec![
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ];

        assert_eq!(6, p2(input.into_iter()))
    }
}
