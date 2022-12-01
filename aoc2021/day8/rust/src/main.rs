use std::{io::Read, str::FromStr, string::ParseError};

/// Ruleset
/// 0-9 : Seven segment digit
/// a-g : The segments
/// =>  : is identified by
/// ->  : gives us
/// a   : is contained in
/// a!  : is unambigously identified
/// -   : take segment of RHS from LHS
/// - a : take segment from LHS
/// ^  : XOR
/// &   : AND
///
/// 1 => 2 digit -> c + f
/// 7 => 3 digit -> c + f + a
/// 7 - 1 -> a!
/// a!
/// 4 => 4 digit -> c + f + b + d
/// 8 => 7 digit
/// 0 => 6 digit with 1 and !4
/// 0 ^ 8 -> d!
/// 4 ^ d! ^ 1 -> b!
/// a! b! d!
/// 9 => 6 segment & 4
/// 9 - 4 - a! -> g!
/// 3 => a! + d! + g! + 1
/// a! b! d! g!
/// 2 => 5 digit with b! and does not contain 1
/// 1 & 2 -> f!
/// 1 ^ f! => c!
/// 2 ^ a! ^ d! ^ c! ^ g! => e!
/// a! b! c! d! e! f! g!
fn main() {
    let digits = read_input();
    let cnt = digits.into_iter().fold(0, |count, line| {
        let wiring = resolve_wiring(line.clone());

        search_output(
            line.into_iter()
                .skip_while(|d| d.store != 0)
                .skip(1)
                .collect(),
            &wiring[..],
        ) + count
    });

    println!("{}", cnt)
}

fn search_output(output: Vec<Digit>, wiring: &[Digit]) -> u64 {
    let mut decoded = vec![];
    for d in output {
        for check in wiring {
            if d.store == check.store {
                decoded.push(*check);
                break;
            }
        }
    }

    let res = assemble_decimial(&decoded[..]);
    println!("{}", res);
    res
}

fn read_input() -> Vec<Vec<Digit>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    input
        .trim()
        .split('\n')
        .map(|line| line.split(' ').map(|s| s.parse().unwrap()).collect())
        .collect()
}

fn resolve_wiring(digits: Vec<Digit>) -> [Digit; 10] {
    let mut resolved = [Digit::default(); 10];
    let _ = digits
        .iter()
        .map(|d| {
            // TODO: skip duplicates
            match d.len {
                // 1
                2 => resolved[1] = *d,
                // 7
                3 => resolved[7] = *d,
                // 4
                4 => resolved[4] = *d,
                // 8
                7 => resolved[8] = *d,
                _ => {}
            }
        })
        .collect::<Vec<()>>();

    let a = resolved[7].xor(&resolved[1]);
    let four = resolved[4];
    // 9
    if let Some(d) = digits.iter().find(|d| d.len == 6 && d.contains(&four)) {
        resolved[9] = *d;
    }

    // 0
    let one = resolved[1];
    if let Some(d) = digits
        .iter()
        .find(|d| d.len == 6 && !d.contains(&four) && d.contains(&one))
    {
        resolved[0] = *d;
    }

    // d
    let d = resolved[0].xor(&resolved[8]);
    // g
    let g = resolved[9].xor(&four).xor(&a);

    // b
    let b = resolved[4].xor(&d).xor(&one);

    // 3
    resolved[3] = one.or(&a).or(&d).or(&g);

    // 2
    if let Some(d) = digits
        .iter()
        .find(|d| d.len == 5 && !d.contains(&b) && !d.contains(&one))
    {
        resolved[2] = *d;
    }

    // c
    let c = resolved[1].and(&resolved[2]);

    // e
    let e = resolved[2].xor(&a).xor(&c).xor(&d).xor(&g);

    // 5
    resolved[5] = resolved[8].xor(&c).xor(&e);
    // 6
    resolved[6] = resolved[8].xor(&c);

    // assign the labels
    for (i, val) in resolved.iter_mut().enumerate() {
        val.label = i as u8;
    }

    resolved
}

fn assemble_decimial(digits: &[Digit]) -> u64 {
    let mut num = 0;
    for (i, d) in digits.iter().rev().enumerate() {
        let val = d.label as u64;
        num += val * 10u64.pow(i as u32);
    }

    num
}

#[derive(Debug, Clone, Copy, Default)]
struct Digit {
    label: u8,
    len: u32,
    /// stores the segment configuration in bits
    /// 0: a
    /// ...
    /// 6: g
    /// 7: unused
    store: u8,
}

impl FromStr for Digit {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut d = Digit::default();
        for b in s.bytes() {
            match b {
                97 => d.store |= 0b0000_0001,
                98 => d.store |= 0b0000_0010,
                99 => d.store |= 0b0000_0100,
                100 => d.store |= 0b0000_1000,
                101 => d.store |= 0b0001_0000,
                102 => d.store |= 0b0010_0000,
                103 => d.store |= 0b0100_0000,
                _ => {}
            }
        }
        d.len = d.store.count_ones();

        Ok(d)
    }
}

impl Digit {
    fn and(&self, other: &Digit) -> Digit {
        let new_val = self.store & other.store;
        let len = new_val.count_ones();
        Digit {
            label: 0,
            len,
            store: new_val,
        }
    }

    fn or(&self, other: &Digit) -> Digit {
        let new_val = self.store | other.store;
        let len = new_val.count_ones();
        Digit {
            label: 0,
            len,
            store: new_val,
        }
    }

    fn xor(&self, other: &Digit) -> Digit {
        let new_val = self.store ^ other.store;
        let len = new_val.count_ones();
        Digit {
            label: 0,
            len,
            store: new_val,
        }
    }

    fn contains(&self, other: &Digit) -> bool {
        (self.store & other.store) == other.store
    }
}
