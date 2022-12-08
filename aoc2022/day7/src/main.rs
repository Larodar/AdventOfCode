use std::{
    io::{stdin, Read},
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
};

static CTR: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let mut buf = String::new();
    _ = stdin().read_to_string(&mut buf).unwrap();

    match std::env::args()
        .into_iter()
        .skip(1)
        .next()
        .map(|s| s.parse::<u32>().unwrap())
    {
        Some(1) => println!("{}", p1(&buf[..])),
        //Some(2) => println!("{}", p2(&buf[..])),
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}

fn p1(input: &str) -> u64 {
    0
}

#[derive(Debug, PartialEq)]
struct FsEntry {
    name: String,
    size: usize,
    kids: Option<Vec<FsEntry>>,
}

impl FsEntry {
    fn with_name(name: String) -> FsEntry {
        FsEntry {
            name,
            size: 0,
            kids: None,
        }
    }

    fn total_size(&self, threshold: usize) -> usize {
        let mut ret = if self.size < threshold { self.size } else { 0 };
        if let Some(kids) = self.kids.as_deref() {
            for entry in kids {
                ret += entry.total_size(threshold);
            }
        }

        ret
    }

    fn from_lines<'a>(mut lines: &'a [&'a str], name: String) -> (&'a [&'a str], Option<FsEntry>) {
        eprintln!("{}", name);
        // if CTR.load(SeqCst) > 10 {
        //     panic!();
        // }
        // CTR.fetch_add(1, SeqCst);
        let mut kids = vec![];
        loop {
            if lines.len() == 0 {
                break;
            } else if lines[0].starts_with("$ ls") {
                let mut i = 1;
                while i < lines.len() && !lines[i].starts_with("$") {
                    let mut parts = lines[i].split(' ');
                    match parts.next().unwrap() {
                        "dir" => {
                            kids.push(FsEntry::with_name(parts.next().unwrap().to_string()));
                        }
                        size => {
                            let size: usize = size.parse().unwrap();
                            let name = parts.next().unwrap().to_string();
                            kids.push(FsEntry {
                                name,
                                size,
                                kids: None,
                            });
                        }
                    }
                    i += 1;
                }
                lines = &lines[i..];
            } else if lines[0].starts_with("$ cd") {
                match parse_cd(lines[0]) {
                    CdAction::Into(s) => {
                        let res = FsEntry::from_lines(&lines[1..], s);
                        lines = res.0;
                        if let Some(entry) = res.1 {
                            kids.push(entry);
                        }
                    }
                    CdAction::Back => {
                        lines = &lines[1..];
                        break;
                    }
                    CdAction::Root => break,
                }
            } else {
                panic!("unknown state");
            }
        }

        let entry = FsEntry {
            name,
            size: 0,
            kids: if kids.len() == 0 { None } else { Some(kids) },
        };

        (lines, Some(entry))
    }
}

fn parse_cd(line: &str) -> CdAction {
    let mut parts = line.split(' ');
    let arg = parts.nth(2).unwrap();
    match arg {
        ".." => CdAction::Back,
        "/" => CdAction::Root,
        a => CdAction::Into(a.to_string()),
    }
}

#[derive(Debug)]
enum CdAction {
    Back,
    Root,
    Into(String),
}

fn parse_fs_tree(input: &str) -> FsEntry {
    let mut root = FsEntry {
        name: "/".to_string(),
        size: 0,
        kids: Some(vec![]),
    };

    let lines = input.lines().collect::<Vec<_>>();
    let mut remaining = &lines[..];
    loop {
        if remaining.is_empty() {
            break;
        } else if remaining[0].starts_with("$ ls") {
            let mut i = 1;
            while remaining.len() > 0 && !remaining[i].starts_with("$") {
                let mut parts = remaining[i].split(' ');
                match parts.next().unwrap() {
                    "dir" => {}
                    size => {
                        let size: usize = size.parse().unwrap();
                        let name = parts.next().unwrap().to_string();
                        root.kids.as_mut().unwrap().push(FsEntry {
                            name,
                            size,
                            kids: None,
                        });
                    }
                }
                i += 1;
            }
            remaining = &remaining[i..];
        } else {
            match parse_cd(remaining[0]) {
                CdAction::Root => remaining = &remaining[1..],
                CdAction::Into(name) => {
                    let ret = FsEntry::from_lines(&remaining[1..], name);
                    remaining = ret.0;
                    if let Some(entry) = ret.1 {
                        root.kids.as_mut().unwrap().push(entry);
                    }
                }
                CdAction::Back => unreachable!("trying to go back from root"),
            }
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1_test() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let expected = FsEntry {
            name: "/".to_owned(),
            size: 0,
            kids: Some(vec![
                FsEntry {
                    name: "a".to_owned(),
                    size: 0,
                    kids: Some(vec![
                        FsEntry {
                            name: "e".to_owned(),
                            size: 0,
                            kids: Some(vec![FsEntry {
                                name: "i".to_owned(),
                                size: 584,
                                kids: None,
                            }]),
                        },
                        FsEntry {
                            name: "f".to_owned(),
                            size: 29116,
                            kids: None,
                        },
                        FsEntry {
                            name: "g".to_owned(),
                            size: 2557,
                            kids: None,
                        },
                        FsEntry {
                            name: "h.lst".to_owned(),
                            size: 62596,
                            kids: None,
                        },
                    ]),
                },
                FsEntry {
                    name: "b.txt".to_owned(),
                    size: 14848514,
                    kids: None,
                },
                FsEntry {
                    name: "c.dat".to_owned(),
                    size: 8504156,
                    kids: None,
                },
                FsEntry {
                    name: "d".to_owned(),
                    size: 0,
                    kids: Some(vec![
                        FsEntry {
                            name: "j".to_owned(),
                            size: 4060174,
                            kids: None,
                        },
                        FsEntry {
                            name: "d.log".to_owned(),
                            size: 8033020,
                            kids: None,
                        },
                        FsEntry {
                            name: "d.ext".to_owned(),
                            size: 5626152,
                            kids: None,
                        },
                        FsEntry {
                            name: "k".to_owned(),
                            size: 7214296,
                            kids: None,
                        },
                    ]),
                },
            ]),
        };

        let result = parse_fs_tree(input);
        dbg!(&expected, &result);
        assert_eq!(expected, result);
    }
}
