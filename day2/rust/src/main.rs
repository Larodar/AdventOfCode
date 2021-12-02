/// I know this is over engineered, but I wanted to practice this parsing pattern.
fn main() {
    let input_path = "/home/larodar/source/AdventOfCode/day2/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut cyclops = Cyclops::default();

    let commands = read_commands(input);
    for c in commands {
        cyclops.execute_command(c);
    }

    println!("{}", cyclops.get_distance());
}

fn read_commands<T: AsRef<str>>(input: T) -> Vec<Command> {
    let mut data = input.as_ref();
    let mut result = vec![];
    while !data.is_empty() {
        data = data.trim_start_matches(is_not_alphanumeric);
        if data.is_empty() {
            return result;
        }

        let (comm, remainder) = Command::parse(data);
        result.push(comm);
        data = remainder;
    }

    result
}

fn is_not_alphanumeric(c: char) -> bool {
    !c.is_alphanumeric()
}

struct Cyclops {
    horizontal: u16,
    depth: i32,
    aim: i32,
}

impl Default for Cyclops {
    fn default() -> Self {
        Cyclops {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl Cyclops {
    pub fn execute_command(&mut self, comm: Command) {
        match comm {
            Command::Forward(val) => {
                self.horizontal += val;
                self.depth += self.aim * val as i32;
            }
            Command::Down(val) => {
                self.aim += val as i32;
            }
            Command::Up(val) => self.aim -= val as i32,
        }
    }

    pub fn get_distance(&self) -> i32 {
        self.horizontal as i32 * self.depth
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Forward(u16),
    Up(u16),
    Down(u16),
}

impl Command {
    fn parse(input: &str) -> (Command, &str) {
        let start = input.chars().take_while(|c| !c.is_alphanumeric()).count();
        let (name, remainder) = Command::parse_name(&input[start..]);
        let remainder = remainder.trim_start_matches(is_not_alphanumeric);
        match name {
            "forward" => Command::parse_forward(remainder),
            "down" => Command::parse_down(remainder),
            "up" => Command::parse_up(remainder),
            _ => panic!("Unknown command."),
        }
    }

    fn parse_name(input: &str) -> (&str, &str) {
        let token_len = input.chars().take_while(|c| c.is_alphabetic()).count();
        (&input[..token_len], &input[token_len..])
    }

    fn parse_forward(input: &str) -> (Command, &str) {
        let (value, remainder) = Command::read_value(input);
        (Command::Forward(value), remainder)
    }

    fn parse_down(input: &str) -> (Command, &str) {
        let (value, remainder) = Command::read_value(input);
        (Command::Down(value), remainder)
    }

    fn parse_up(input: &str) -> (Command, &str) {
        let (value, remainder) = Command::read_value(input);
        (Command::Up(value), remainder)
    }

    fn read_value(input: &str) -> (u16, &str) {
        let value_len = input.chars().take_while(|c| c.is_digit(10)).count();
        (input[..value_len].parse().unwrap(), &input[value_len..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_parse() {
        let input = "4";
        assert_eq!(Command::parse_forward(input), (Command::Forward(4), ""));

        let input = "4\n";
        assert_eq!(Command::parse_forward(input), (Command::Forward(4), "\n"));

        let input = "4\ndown";
        assert_eq!(
            Command::parse_forward(input),
            (Command::Forward(4), "\ndown")
        );
    }

    #[test]
    fn down_parse() {
        let input = "4";
        assert_eq!(Command::parse_down(input), (Command::Down(4), ""));

        let input = "4\n";
        assert_eq!(Command::parse_down(input), (Command::Down(4), "\n"));

        let input = "4\ndown";
        assert_eq!(Command::parse_down(input), (Command::Down(4), "\ndown"));
    }

    #[test]
    fn up_parse() {
        let input = "4";
        assert_eq!(Command::parse_up(input), (Command::Up(4), ""));

        let input = "4\n";
        assert_eq!(Command::parse_up(input), (Command::Up(4), "\n"));

        let input = "4\ndown";
        assert_eq!(Command::parse_up(input), (Command::Up(4), "\ndown"));
    }

    #[test]
    fn name_parse() {
        let input = "forward";
        assert_eq!(Command::parse_name(input), ("forward", ""));
        let input = "up 4";
        assert_eq!(Command::parse_name(input), ("up", " 4"));
        let input = "down 4\n";
        assert_eq!(Command::parse_name(input), ("down", " 4\n"));
        let input = "forward 5\ndown 3";
        assert_eq!(Command::parse_name(input), ("forward", " 5\ndown 3"));
    }

    #[test]
    fn command_parse() {
        let input = "forward 5\ndown";
        assert_eq!(Command::parse(input), (Command::Forward(5), "\ndown"));

        let input = "up 5\ndown 2";
        assert_eq!(Command::parse(input), (Command::Up(5), "\ndown 2"));

        let input = "forward 5\n";
        assert_eq!(Command::parse(input), (Command::Forward(5), "\n"));

        let input = "down 5";
        assert_eq!(Command::parse(input), (Command::Down(5), ""));
    }

    #[test]
    fn data_set_parse() {
        let input = concat!(
            "forward 5\n",
            "down 5\n",
            "forward 8\n",
            "up 3\n",
            "down 8\n",
            "forward 2\n",
        );

        let expected = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        assert_eq!(read_commands(input), expected);

        let input = concat!(
            "forward 5\n",
            "down 5\n",
            "forward 8\n",
            "up 3\n",
            "down 8\n",
            "forward 2",
        );

        let expected = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        assert_eq!(read_commands(input), expected);
    }
}
