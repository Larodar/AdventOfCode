use std::{
    collections::HashMap,
    io::BufRead,
    ops::{Deref, DerefMut},
};

fn main() {
    let input = read_input();
    let paths = find_paths(input);
    println!("{}", paths.len());
}

fn read_input() -> CaveSys {
    let mut connections = vec![];
    let mut caves = Caves(HashMap::new());
    for l in std::io::stdin().lock().lines() {
        let line = l.unwrap();
        let (one, two, con) = parse_connection(line.as_str());
        connections.push(con);
        register_cave(&mut caves, one);
        register_cave(&mut caves, two);
    }

    CaveSys::new(caves, connections)
}

fn register_cave(caves: &mut Caves, cave: (String, CaveKind)) {
    let name = cave.0;
    if caves.get(&name).is_none() {
        caves.insert(name, cave.1);
    }
}

fn find_paths(env: CaveSys) -> Vec<Path> {
    let start_name = "start".to_string();
    let base = vec!["start".to_string()].into();
    walk(start_name.as_str(), &env, &base)
}

fn get_connections<'a>(cave_name: &str, connections: &'a [Connection]) -> Vec<&'a str> {
    connections
        .iter()
        .filter_map(|con| {
            if con.one.as_str() == cave_name {
                Some(con.two.as_str())
            } else if con.two.as_str() == cave_name {
                Some(con.one.as_str())
            } else {
                None
            }
        })
        .collect()
}

fn walk(current: &str, env: &CaveSys, base: &Path) -> Vec<Path> {
    let connections = &env.connections;
    let mut ret = vec![];
    for con in get_connections(current, connections) {
        if con == "end" {
            let mut path = base.to_vec();
            path.push(con.to_string());
            ret.push(path.into());
            continue;
        }

        if con == "start" {
            continue;
        }

        let in_path = base
            .iter()
            .any(|s| s.as_str() == con && s.chars().next().unwrap().is_lowercase());
        if base.1 && in_path {
            continue;
        }

        let mut branch = Path(base.to_vec(), base.1 || in_path);
        branch.push(con.to_string());
        for path in walk(con, env, &branch) {
            ret.push(path);
        }
    }

    ret
}

fn parse_connection(line: &str) -> ((String, CaveKind), (String, CaveKind), Connection) {
    let mut parts = line.split('-');
    let cave_one = parse_cave(parts.next().unwrap());
    let cave_two = parse_cave(parts.next().unwrap());
    let con = Connection {
        one: cave_one.0.clone(),
        two: cave_two.0.clone(),
    };

    (cave_one, cave_two, con)
}

fn parse_cave(cave: &str) -> (String, CaveKind) {
    let kind = match cave {
        "start" => CaveKind::Start,
        "end" => CaveKind::End,
        s => {
            let c = s.chars().next().unwrap();
            if c.is_lowercase() {
                CaveKind::Small
            } else {
                CaveKind::Big
            }
        }
    };

    (cave.to_string(), kind)
}

#[derive(Debug, PartialEq, Clone)]
struct CaveSys {
    caves: Caves,
    connections: Vec<Connection>,
}

impl CaveSys {
    pub fn new(caves: Caves, connections: Vec<Connection>) -> CaveSys {
        CaveSys { caves, connections }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Path(Vec<String>, bool);

impl Deref for Path {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<String>> for Path {
    fn from(v: Vec<String>) -> Self {
        Path(v, false)
    }
}

#[derive(Debug, PartialEq, Clone)]
enum CaveKind {
    Start,
    End,
    Small,
    Big,
}

#[derive(Debug, PartialEq, Clone)]
struct Caves(HashMap<String, CaveKind>);

impl Deref for Caves {
    type Target = HashMap<String, CaveKind>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Caves {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Connection {
    one: String,
    two: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cave_parsing() {
        let input = "a";
        assert_eq!(parse_cave(input), (input.to_string(), CaveKind::Small));

        let input = "A";
        assert_eq!(parse_cave(input), (input.to_string(), CaveKind::Big));
        let input = "start";
        assert_eq!(parse_cave(input), (input.to_string(), CaveKind::Start,));
        let input = "end";
        assert_eq!(parse_cave(input), (input.to_string(), CaveKind::End));
    }

    #[test]
    fn connection_parsing() {
        let input = "a-end";
        let exp_one = ("a".to_string(), CaveKind::Small);
        let exp_two = ("end".to_string(), CaveKind::End);
        let exp_con = Connection {
            one: "a".to_string(),
            two: "end".to_string(),
        };

        let (one, two, con) = parse_connection(input);
        assert_eq!(exp_con, con);
        assert_eq!(exp_one, one);
        assert_eq!(exp_two, two);
    }

    #[test]
    fn walk_through() {
        let input = vec!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
        let mut connections = vec![];
        let mut caves = Caves(HashMap::new());
        for l in input {
            let (one, two, con) = parse_connection(l);
            connections.push(con);
            register_cave(&mut caves, one);
            register_cave(&mut caves, two);
        }

        let cavesys = CaveSys::new(caves, connections);
        let paths = find_paths(cavesys);
        assert_eq!(paths.len(), 36);
    }
}
