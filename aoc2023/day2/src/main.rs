use std::io::{stdin, BufRead, Read};

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

fn p1(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    input.map(|l| Game::from_line(l.as_ref())).filter(|g| 
        g.red <= 12 && g.green <= 13 && g.blue <= 14
    ).fold(0, |total, g| total + g.id) as u64
}

fn p2(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    input.map(|l| Game::from_line(l.as_ref())).map(|g| 
        g.red * g.green * g.blue
    ).sum()
}

#[derive(Debug)]
struct Game {
    id: usize,
    red: u64,
    green: u64,
    blue: u64,
}

impl Game {
    fn from_line(line: &str) -> Game {
        let Some((game_info, remainder)) = line.split_once(':') else {
            panic!("invalid format")
        };
        let id = game_info.split_once(' ').unwrap().1.parse().unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for info in remainder.trim().split(|c| c == ';' || c == ',') {
            let (count, color) = info.trim().split_once(' ').unwrap();
            match color {
                "red" => red = std::cmp::max(red, count.parse().unwrap()),
                "green" => green = std::cmp::max(green, count.parse().unwrap()),
                "blue" => blue = std::cmp::max(blue, count.parse().unwrap()),
                color => panic!("unhandled color: {color}")
            }
        }

        Game {
            id,
            red,
            green,
            blue,
        }
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn should_parse_game() {}
}
