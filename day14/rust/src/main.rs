use std::{collections::HashMap, io::BufRead, str::FromStr};

fn main() {
    let (input, rules) = read_input();

    let mut rule_set = HashMap::new();

    for rule in rules.iter() {
        let _ = rule_set.insert(rule.0.as_str(), rule.1.clone());
    }

    let mut state = State {
        rules: rule_set,
        cache: HashMap::new(),
        heuristic_cache: HashMap::new(),
    };

    let exp_20 = expand_20(input.as_str(), &mut state);
    let result = expand_40(&exp_20, &mut state);

    println!("{}", calc_result(&result));
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

#[derive(Debug)]
struct Rule(String, String);
impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" -> ");
        let input = split.next().unwrap().to_string();
        let output = split.next().unwrap().to_string();
        Ok(Rule(input, output))
    }
}

struct State<'a> {
    rules: HashMap<&'a str, String>,
    cache: HashMap<String, String>,
    heuristic_cache: HashMap<String, HashMap<u8, usize>>,
}

impl<'a> State<'a> {
    fn lookup(&self, slice: &str) -> Option<&String> {
        self.rules.get(slice)
    }

    fn build_str_heuristics(&mut self, input: &str) -> HashMap<u8, usize> {
        let mut ret = HashMap::new();
        // do not count the last char, since it would be counted twice
        for c in input.bytes() {
            if let Some(val_ref) = ret.get_mut(&c) {
                *val_ref += 1;
            } else {
                ret.insert(c, 1);
            }
        }

        ret
    }

    fn add_to_cache(&mut self, key: String, val: &str) {
        let heuristic = self.build_str_heuristics(&val[..val.len() - 1]);
        self.heuristic_cache.insert(key.clone(), heuristic);
        self.cache.insert(key, val.to_string());
    }
}

fn fill(input: &str, state: &mut State, depth: usize) {
    let result = expand(input, state);
    if depth <= 0 {
        state.build_str_heuristics(result.as_str());
        return;
    }

    if result.len() > 4096 * 4 {
        // divide if we expanded above the threshhold.
        let mut last_char = '\0';
        let mut offset = 0;
        while offset + 4 < result.len() {
            let start = offset;
            let end = offset + 4;
            offset = end;
            if last_char != '\0' {
                let mut s = String::with_capacity(1);
                s.push(last_char);
                state.build_str_heuristics(s.as_str());
            }

            let slice = &result[start..end];
            fill(slice, state, depth - 1);
            last_char = slice.chars().last().unwrap();
        }

        let slice = &result[offset..];
        fill(slice, state, depth - 1);

        return;
    }

    fill(result.as_str(), state, depth - 1);
}

fn expand_20(input: &str, state: &mut State) -> String {
    let mut ret = String::new();
    for i in 0..input.len() - 1 {
        let window = &input[i..i + 2];
        let key = window.to_string();
        let mut temp = key.clone();
        for _ in 0..20 {
            temp = expand(temp.as_str(), state);
        }

        ret.push_str(&temp[..temp.len() - 1]);
        state.add_to_cache(key.clone(), temp.as_str());
    }

    ret.push(input.chars().last().unwrap());

    ret
}

fn expand_40(input: &str, state: &mut State) -> HashMap<u8, usize> {
    let mut ret = HashMap::new();

    for i in 0..input.len() - 1 {
        let window = &input[i..i + 2];
        if let Some(s) = state.cache.get(window) {
            if i == input.len() - 2 {
                // last segment
                let last_char = &s[s.len() - 1..];
                add_to_heuristic(&mut ret, last_char);
            }
        } else {
            let s = expand_20(window, state);
            state.add_to_cache(window.to_string(), s.as_str());
            if i == input.len() - 2 {
                // last segment
                let last_char = &s[s.len() - 1..];
                add_to_heuristic(&mut ret, last_char);
            }
        }

        let heuristic = state.heuristic_cache.get(window).unwrap();
        for (k, v) in heuristic.iter() {
            if let Some(b) = ret.get_mut(k) {
                *b += *v;
            } else {
                ret.insert(*k, *v);
            }
        }
    }

    ret
}

fn add_to_heuristic(heurisitic: &mut HashMap<u8, usize>, val: &str) {
    let last = &val[val.len() - 1..];
    for c in last.bytes() {
        if let Some(val_ref) = heurisitic.get_mut(&c) {
            *val_ref += 1;
        } else {
            heurisitic.insert(c, 1);
        }
    }
}

fn expand(input: &str, state: &mut State) -> String {
    let mut ret = String::with_capacity(input.len() * 2);
    for i in 0..input.len() - 1 {
        let window = &input[i..i + 2];
        let mut chars = window.chars();
        if ret.is_empty() {
            ret.push(chars.next().unwrap());
        } else {
            let _ = chars.next();
        }

        if let Some(s) = state.lookup(window) {
            ret.push_str(s.as_str());
        }

        ret.push(chars.next().unwrap());
    }

    ret
}

fn calc_result(heuristic: &HashMap<u8, usize>) -> u64 {
    let max = heuristic.values().max().unwrap();
    let min = heuristic.values().min().unwrap();

    (max - min) as u64
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
        let rules = lines_iter
            .into_iter()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<Rule>>();

        let mut rule_set = HashMap::new();

        for rule in rules.iter() {
            let _ = rule_set.insert(rule.0.as_str(), rule.1.clone());
        }

        let mut state = State {
            rules: rule_set,
            cache: HashMap::new(),
            heuristic_cache: HashMap::new(),
        };

        let exp_20 = expand_20(input, &mut state);
        let result = expand_40(&exp_20, &mut state);

        //assert_eq!(calc_result(&result), 1588);
        assert_eq!(calc_result(&result), 2188189693529);
    }

    #[test]
    fn test_expand() {
        let rules = build_rules();
        let mut state = build_state(&rules);

        let in_str = "NNCB";
        let result = expand(in_str, &mut state);

        assert_eq!(result.as_str(), "NCNBCHB");

        let in_str = "NNC";
        let result = expand(in_str, &mut state);

        assert_eq!(result.as_str(), "NCNBC");
    }

    //#[test]
    fn test_multiple_expansion() {
        let rules = build_rules();
        //let mut ref_state = build_state(&rules);
        let mut state = build_state(&rules);
        let mut expected = "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".to_owned();
        for _ in 0..6 {
            expected = expand(&expected.as_str(), &mut state);
        }

        assert_eq!(expected.len(), 3073);

        let result_1 = expand_20("NN", &mut state);
        let result_2 = expand_20("NC", &mut state);
        let result_3 = expand_20("CB", &mut state);
        let mut result = String::with_capacity(result_1.len() * 3);
        result.push_str(&result_1.as_str()[..result_1.len() - 1]);
        result.push_str(&result_2.as_str()[..result_2.len() - 1]);
        result.push_str(result_3.as_str());
        assert_eq!(result, expected);

        let heuristic = state.build_str_heuristics(result.as_str());
        assert_eq!(*heuristic.get(&b'B').unwrap(), 1749);
        assert_eq!(*heuristic.get(&b'N').unwrap(), 865);
        assert_eq!(*heuristic.get(&b'C').unwrap(), 298);
        assert_eq!(*heuristic.get(&b'H').unwrap(), 161);
    }

    fn build_rules() -> Vec<Rule> {
        vec![
            "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C", "NN -> C",
            "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N", "CN -> C",
        ]
        .into_iter()
        .map(|l| l.parse().unwrap())
        .collect()
    }

    fn build_state<'a>(rules: &'a Vec<Rule>) -> State<'a> {
        let mut rule_set = HashMap::new();

        for rule in rules.iter() {
            let _ = rule_set.insert(rule.0.as_str(), rule.1.clone());
        }
        State {
            rules: rule_set,
            cache: HashMap::new(),
            heuristic_cache: HashMap::new(),
        }
    }
}
