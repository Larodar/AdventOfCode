use std::io::BufRead;

fn main() {
    let std_in = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let (_chunks, score) = read_data_set(&std_in[..]);
    println!("{}", score);
}

fn read_data_set<T: AsRef<str>>(input: &[T]) -> (Vec<Chunk>, u64) {
    let mut error_score = 0;
    let mut chunks = vec![];
    for line in input {
        let (c, score) = read_chunks(line.as_ref());
        c.into_iter().for_each(|el| chunks.push(el));
        error_score += score;
    }

    (chunks, error_score)
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

fn read_chunks(input: &str) -> (Vec<Chunk>, u64) {
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
                remainder = &remainder[1..];
                continue;
            }
        };
        match Chunk::parse(remainder, kind) {
            Ok((chunk, rest)) => {
                chunks.push(chunk);
                remainder = rest;
            }
            Err(ParseChunkError(ParseChunkErrorKind::Incomplete)) => {
                remainder = "";
            }
            Err(ParseChunkError(ParseChunkErrorKind::Corrupt(c))) => {
                error_score += get_char_score(c);
                remainder = "";
            }
        }
    }

    (chunks, error_score)
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
}

impl Chunk {
    pub fn parse(input: &str, kind: ChunkKind) -> ParseChunkResult {
        if input.is_empty() {
            return Err(ParseChunkError(ParseChunkErrorKind::Incomplete));
        }

        let mut remainder = input;
        let mut kids = vec![];

        loop {
            if remainder.is_empty() {
                break;
            }

            let child_kind = match remainder.chars().next().unwrap() {
                '(' => Some(ChunkKind::Parenthesis),
                '{' => Some(ChunkKind::Braces),
                '[' => Some(ChunkKind::Brackets),
                '<' => Some(ChunkKind::Angles),
                c => {
                    if c != kind.get_closing_token() {
                        return Err(ParseChunkError(ParseChunkErrorKind::Corrupt(c)));
                    }

                    remainder = &remainder[1..];
                    None
                }
            };

            if let Some(kind) = child_kind {
                let (child, rest) = Chunk::parse(&remainder[1..], kind)?;
                kids.push(child);
                remainder = rest;
            } else {
                break;
            }
        }

        let ret = Chunk {
            kind,
            kids: if kids.is_empty() { None } else { Some(kids) },
        };

        Ok((ret, remainder))
    }
}

type ParseChunkResult<'a> = Result<(Chunk, &'a str), ParseChunkError>;

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

        let (_, score) = read_data_set(&input[..]);
        assert_eq!(score, 26397);
    }
}
