use std::str::Lines;

fn main() {
    let input_str =
        std::fs::read_to_string("/home/v1p3r/source/AdventOfCode/day4/input.txt").unwrap();

    let (stack, mut boards) = parse_game_data(input_str);
    let mut last_to_win = 0;
    for num in stack {
        for board in boards.iter_mut().filter(|b| !b.won) {
            board.mark(num);
            if let Some(sum) = board.check_for_win() {
                last_to_win = sum * num as u64;
            }
        }
    }

    println!("{}", last_to_win);
}

#[derive(Debug)]
struct Board {
    entries: Vec<(u8, bool)>,
    side_length: usize,
    won: bool,
}

impl Board {
    pub fn new(entries: Vec<(u8, bool)>, side_length: usize) -> Board {
        Board {
            entries,
            side_length,
            won: false,
        }
    }

    fn mark(&mut self, element: u8) {
        for entry in self.entries.iter_mut() {
            if entry.0 == element {
                entry.1 = true;
                break;
            }
        }
    }

    fn check_for_win(&mut self) -> Option<u64> {
        for n in 0..self.side_length {
            if self.check_nth_row(n) || self.check_nth_column(n) {
                self.won = true;
                return Some(self.sum_unmarked());
            }
        }

        None
    }

    fn sum_unmarked(&self) -> u64 {
        self.entries.iter().fold(0, |sum, current| {
            if current.1 {
                sum
            } else {
                sum + current.0 as u64
            }
        })
    }

    fn check_nth_row(&self, n: usize) -> bool {
        let offset = n * self.side_length;
        self.entries
            .iter()
            .skip(offset)
            .take(self.side_length)
            .map(|v| v.1)
            .reduce(|current, last| last && current)
            .unwrap()
    }

    fn check_nth_column(&self, n: usize) -> bool {
        let offset = n;
        self.entries
            .iter()
            .skip(offset)
            .step_by(self.side_length)
            .map(|v| v.1)
            .reduce(|current, last| last && current)
            .unwrap()
    }
}

fn parse_game_data<T: AsRef<str>>(input: T) -> (Vec<u8>, Vec<Board>) {
    let mut lines = input.as_ref().lines();
    let stack = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut boards = vec![];
    while let Some(b) = read_board(&mut lines) {
        boards.push(b);
    }

    (stack, boards)
}

fn read_board(lines: &mut Lines) -> Option<Board> {
    // read first line to determine how many lines to read
    let (len, mut board) = loop {
        break if let Some(line) = lines.next() {
            if line.is_empty() {
                continue;
            }

            let items = line
                .split(' ')
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<u8>>();
            (items.len(), items)
        } else {
            return None;
        };
    };

    // read rest of the board
    for _ in 1..len {
        if let Some(line) = lines.next() {
            line.split(' ')
                .filter_map(|s| s.parse().ok())
                .for_each(|v| board.push(v));
        } else {
            panic!("Invalid board structure.");
        }
    }

    Some(Board::new(
        board.into_iter().map(|v| (v, false)).collect(),
        len,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_parsing() {
        let input = concat!(
            "22 13 17 11  0\n",
            " 8  2 23  4 24\n",
            "21  9 14 16  7\n",
            " 6 10  3 18  5\n",
            " 1 12 20 15 19\n",
        );

        let data = vec![
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ];

        let expected = Board::new(data.into_iter().map(|v| (v, false)).collect(), 5);
        let board = read_board(&mut input.lines());
        assert_eq!(board.unwrap().entries, expected.entries);
    }

    #[test]
    fn game_data_parsing() {
        let input = concat!(
            "1,2,3,4,5,6,7\n",
            "22 13 17 11  0\n",
            " 8  2 23  4 24\n",
            "21  9 14 16  7\n",
            " 6 10  3 18  5\n",
            " 1 12 20 15 19\n",
            "\n",
            "24 13 17 11  0\n",
            " 8  2 23  4 24\n",
            "21  9 14 16  7\n",
            " 6 10  3 18  5\n",
            " 1 12 20 15 19\n",
        );

        let data = vec![
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ];
        let first = Board::new(data.into_iter().map(|v| (v, false)).collect(), 5);
        let data = vec![
            24, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ];
        let second = Board::new(data.into_iter().map(|v| (v, false)).collect(), 5);

        let exp_stack = vec![1, 2, 3, 4, 5, 6, 7];
        let (stack, boards) = parse_game_data(input);
        assert_eq!(stack, exp_stack);
        assert_eq!(boards[0].entries, first.entries);
        assert_eq!(boards[1].entries, second.entries);
    }
}
