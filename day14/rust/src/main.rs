use std::{collections::HashMap, io::BufRead, str::FromStr};

fn main() {
    let (mut input, rules) = read_input();
    let mut string = input[..2].to_string();
    for i in 0..40 {
        string = fill(&string, &rules[..]);
        println!("{}", i);
    }

    let result = calc_result(string.as_str());
    println!("{}", result);
}

fn read_input() -> (String, Vec<Rule>) {
    let stdin = std::io::stdin();
    let mut lines_iter = stdin.lock().lines();
    let input = lines_iter.next().unwrap().unwrap();
    // empty line
    let _ = lines_iter.next();

    let mut rules = vec![];
    for l in lines_iter {
        let line = l.unwrap();
        rules.push(line.parse().unwrap());
    }

    (input, rules)
}

struct Rule(String, String);
impl Rule {
    pub fn matches(&self, other: &str) -> Option<String> {
        if other == self.0.as_str() {
            Some(self.1.clone())
        } else {
            None
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" -> ");
        let input = split.next().unwrap().to_string();
        let output = split.next().unwrap().to_string();
        Ok(Rule(input, output))
    }
}

fn fill(input: &str, rules: &[Rule]) -> String {
    let mut ret = String::with_capacity(input.len() * 2);
    for i in 0..input.len() - 1 {
        let window = &input[i..i + 2];
        if ret.is_empty() {
            ret.push(window.chars().next().unwrap());
        }
        if let Some(s) = get_insertion_char(window, rules) {
            ret.push_str(s.as_str());
        }
        ret.push(window.chars().nth(1).unwrap());
    }

    ret
}

fn get_str_heuristics(input: &str) -> HashMap<u8, usize> {
    let mut ret = HashMap::new();
    for c in input.bytes() {
        if let Some(val_ref) = ret.get_mut(&c) {
            *val_ref += 1;
        } else {
            ret.insert(c, 1);
        }
    }

    ret
}

fn calc_result(input: &str) -> u64 {
    let heuristics = get_str_heuristics(input);
    let max = heuristics.values().max().unwrap();
    let min = heuristics.values().min().unwrap();

    (max - min) as u64
}

fn get_insertion_char(slice: &str, rules: &[Rule]) -> Option<String> {
    for rule in rules {
        let match_opt = rule.matches(slice);
        if match_opt.is_some() {
            return match_opt;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![
            "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B",
            "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B",
            "CC -> N", "CN -> C",
        ];

        let mut lines_iter = input.into_iter();
        let input = lines_iter.next().unwrap();
        // empty line
        let _ = lines_iter.next();

        let mut rules: Vec<Rule> = vec![];
        for l in lines_iter {
            rules.push(l.parse().unwrap());
        }

        let result = fill(input, &rules[..]);
        assert_eq!(result.as_str(), "NCNBCHB");
        let result = fill(result.as_str(), &rules[..]);
        assert_eq!(result.as_str(), "NBCCNBBBCBHCB");
        let result = fill(result.as_str(), &rules[..]);
        assert_eq!(result.as_str(), "NBBBCNCCNBBNBNBBCHBHHBCHB");

        let mut result = fill(result.as_str(), &rules[..]);
        assert_eq!(
            result.as_str(),
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );

        for _ in 0..6 {
            result = fill(result.as_str(), &rules[..]);
        }

        assert_eq!(result.len(), 3073);
        assert_eq!(calc_result(result.as_str()), 1588);
    }
}
