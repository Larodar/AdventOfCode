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
/// !&  : NAND
/// &   : AND
///
/// 1 => 2 digit -> c + f
/// 7 => 3 digit -> c + f + a!
/// a!
/// 4 => 4 digit -> c + f + b + d
/// 8 => 7 digit
/// 0 => 6 digit with 1 and !4
/// 0 !& 8 -> d!
/// 4 & d! -> b!
/// a! b! d!
/// 9 => 6 segment & 4
/// 9 - 4 - a! -> g!
/// 3 => a! + d! + g! + 1
/// a! b! d! g!
/// 2 => 5 digit which is not 3
/// 1 - 2 => f!
/// 1 - f! => c!
/// 2 - a! - d! - c! - g! => e!
/// a! b! c! d! e! f! g!
fn main() {
    let digits = read_input();
    let cnt = digits.into_iter().fold(0, |count, line| {
        let sample = line.iter().take_while(|d| d.store == 0).copied().collect();
        let wiring = resolve_wiring(sample);

        search_output(
            line.into_iter().skip_while(|d| d.store != 0).collect(),
            &wiring[..],
        ) + count
    });

    println!("{}", cnt)
}

fn search_output(output: Vec<Digit>, wiring: &[Digit]) -> i32 {
    let mut sum = 0;
    for d in output {
        if d.store == wiring[1].store
            || d.store == wiring[3].store
            || d.store == wiring[4].store
            || d.store == wiring[8].store
        {
            sum += 1;
        }
    }

    sum
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

    for (i, val) in resolved.iter_mut().enumerate() {
        val.label = i as u8;
    }

    resolved
}

#[derive(Debug, Clone, Copy, Default)]
struct Number {
    label: u8,
    digits: [Digit; 7],
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

    fn contains(&self, other: &Digit) -> bool {
        self.store & other.store == self.store
    }
}
