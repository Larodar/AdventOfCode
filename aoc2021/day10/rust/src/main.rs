use std::io::BufRead;

fn main() {
    let std_in = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let (chunks, _) = read_data_set(&std_in[..]);

    let mut scores = vec![];
    for line in chunks {
        let incomplete = line.iter().filter(|c| !c.complete).collect::<Vec<_>>();
        for c in incomplete {
            let completion = c.get_completion_string();
            let mut score = 0;
            for ch in completion.chars() {
                score *= 5;
                score += get_char_completion_score(ch);
            }

            scores.push(score);
        }
    }

    scores.sort();
    let middle = scores.len() / 2;

    println!("{}", scores[middle]);
}

fn read_data_set<T: AsRef<str>>(input: &[T]) -> (Vec<Vec<Chunk>>, u64) {
    let mut chunks = vec![];
    //let mut error_score = 0;
    for line in input {
        match read_chunks(line.as_ref()) {
            Ok((c, _)) => {
                chunks.push(c);
            }
            Err(_) => {
                // filter out
                continue;
            }
        }
    }

    (chunks, 0)
}

fn get_char_completion_score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("bad char"),
    }
}

fn get_char_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("bad char"),
    }
}

fn read_chunks(input: &str) -> ParseChunkResult<(Vec<Chunk>, u64)> {
    let mut remainder = input;
    let mut chunks = vec![];
    let mut error_score = 0;
    while !remainder.is_empty() {
        let kind = match remainder.chars().next().unwrap() {
            '(' => ChunkKind::Parenthesis,
            '{' => ChunkKind::Braces,
            '[' => ChunkKind::Brackets,
            '<' => ChunkKind::Angles,
            c => {
                error_score += get_char_score(c);
                continue;
            }
        };

        remainder = &remainder[1..];
        let (chunk, rest) = Chunk::parse(remainder, kind)?;
        chunks.push(chunk);
        remainder = rest;
    }

    Ok((chunks, error_score))
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ChunkKind {
    Parenthesis,
    Angles,
    Brackets,
    Braces,
}

impl ChunkKind {
    pub fn get_closing_token(&self) -> char {
        match *self {
            ChunkKind::Angles => '>',
            ChunkKind::Parenthesis => ')',
            ChunkKind::Brackets => ']',
            ChunkKind::Braces => '}',
        }
    }
}

#[derive(Debug)]
struct Chunk {
    kind: ChunkKind,
    kids: Option<Vec<Chunk>>,
    complete: bool,
}

impl Chunk {
    pub fn parse(input: &str, kind: ChunkKind) -> ParseChunkResult<(Chunk, &str)> {
        if input.is_empty() {
            return Ok((
                Chunk {
                    kind,
                    kids: None,
                    complete: false,
                },
                "",
            ));
        }

        let mut remainder = input;
        let mut kids = vec![];

        let mut self_complete = false;
        loop {
            if remainder.is_empty() {
                break;
            }

            let kind = match remainder.chars().next().unwrap() {
                '(' => ChunkKind::Parenthesis,
                '{' => ChunkKind::Braces,
                '[' => ChunkKind::Brackets,
                '<' => ChunkKind::Angles,
                c => {
                    if c != kind.get_closing_token() {
                        return Err(ParseChunkError(ParseChunkErrorKind::Corrupt(c)));
                    }

                    self_complete = true;
                    remainder = &remainder[1..];
                    break;
                }
            };

            let (child, rest) = Chunk::parse(&remainder[1..], kind)?;
            kids.push(child);
            remainder = rest;
        }

        let complete = self_complete && kids.iter().all(|k| k.complete);
        let ret = Chunk {
            kind,
            kids: if kids.is_empty() { None } else { Some(kids) },
            complete,
        };

        Ok((ret, remainder))
    }

    pub fn get_completion_string(&self) -> String {
        let mut ret = match &self.kids {
            Some(k) => {
                let incomplete = k.iter().filter(|kid| !kid.complete).collect::<Vec<_>>();
                if incomplete.len() > 1 {
                    panic!("More than one incomplete chunk?");
                }
                if incomplete.is_empty() {
                    String::new()
                } else {
                    incomplete[0].get_completion_string()
                }
            }
            None => String::new(),
        };

        let c = match self.kind {
            ChunkKind::Angles => '>',
            ChunkKind::Parenthesis => ')',
            ChunkKind::Braces => '}',
            ChunkKind::Brackets => ']',
        };

        ret.push(c);
        ret
    }
}

type ParseChunkResult<T> = Result<T, ParseChunkError>;

#[derive(Debug, PartialEq)]
struct ParseChunkError(ParseChunkErrorKind);

#[derive(Debug, PartialEq)]
enum ParseChunkErrorKind {
    Incomplete,
    Corrupt(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let input = ")";
        let res = Chunk::parse(input, ChunkKind::Parenthesis).unwrap();
        assert_eq!(res.1, "");
        assert!(res.0.kids.is_none());
        assert_eq!(res.0.kind, ChunkKind::Parenthesis);
    }

    #[test]
    fn parse_err() {
        let input = "}";
        let res = Chunk::parse(input, ChunkKind::Parenthesis).unwrap_err();
        assert_eq!(res, ParseChunkError(ParseChunkErrorKind::Corrupt('}')));
    }

    #[test]
    fn parse() {
        let input = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];

        let (chunks, score) = read_data_set(&input[..]);
        let completion = chunks
            .into_iter()
            .map(|c| c.iter().all(|c| c.complete))
            .collect::<Vec<_>>();

        assert_eq!(completion, vec![false, false, false, false, false,])
        //assert_eq!(score, 26397);
    }

    #[test]
    fn completion() {
        let input = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "(((({<>}<{<{<>}{[]{[]{}",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];

        let (chunks, _) = read_data_set(&input[..]);
        let mut scores = vec![];
        for line in chunks {
            let incomplete = line.iter().filter(|c| !c.complete).collect::<Vec<_>>();
            for c in incomplete {
                dbg!(c);
                let completion = c.get_completion_string();
                println!("{}", completion);
                let mut score = 0;
                for ch in completion.chars() {
                    score *= 5;
                    score += get_char_completion_score(ch);
                }

                scores.push(score);
            }
        }

        assert_eq!(scores.len(), 5);
        dbg!(&scores);
        assert!(scores.contains(&288957));
        assert!(scores.contains(&5566));
        assert!(scores.contains(&1480781));
        assert!(scores.contains(&995444));
        assert!(scores.contains(&294));
    }
}
