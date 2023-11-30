use std::io::{stdin, Read};

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
        Some(2) => println!("{}", p2(&buf[..])),
        _ => panic!("specify the part of the solution to run (1/2)"),
    }
}

fn p2(input: &str) -> u64 {
    const FS_SIZE: u64 = 70000000;
    const MIN_REQUIRED: u64 = 30000000;
    let fs = FsEntry::from_lines(&mut input.lines());

    let mut used = fs.total_size() as u64;
    let to_free = MIN_REQUIRED - (FS_SIZE - used);
    if fs.kids.is_some() {
        walk_p2(&fs, &mut used, to_free);
        used
    } else if fs.size < 100_000 {
        fs.size as u64
    } else {
        panic!("This case was not expected.")
    }
}

fn walk_p2(fs: &FsEntry, free_size: &mut u64, required: u64) {
    assert_eq!(fs.size, 0);
    for dir in fs
        .kids
        .as_ref()
        .unwrap()
        .iter()
        .filter(|d| d.kids.is_some())
    {
        let dir_total = dir.total_size() as u64;
        if dir_total > required {
            *free_size = std::cmp::min(*free_size, dir_total);
        }

        walk_p2(dir, free_size, required);
    }
}

fn p1(input: &str) -> u64 {
    let fs = FsEntry::from_lines(&mut input.lines());
    if fs.kids.is_some() {
        let mut total = 0;
        walk_p1(&fs, &mut total);
        total as u64
    } else if fs.size < 100_000 {
        fs.size as u64
    } else {
        panic!("This case was not expected.")
    }
}

fn walk_p1(fs: &FsEntry, total: &mut usize) {
    assert_eq!(fs.size, 0);
    for dir in fs
        .kids
        .as_ref()
        .unwrap()
        .iter()
        .filter(|d| d.kids.is_some())
    {
        let dir_total = dir.total_size();
        if dir_total < 100_000 {
            *total += dir_total;
        }

        walk_p1(dir, total);
    }
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

    fn total_size(&self) -> usize {
        assert!(self.size > 0 && self.kids.is_none() || self.size == 0);
        if self.size > 0 {
            return self.size;
        }

        let mut ret = 0;
        if let Some(kids) = self.kids.as_deref() {
            for entry in kids {
                ret += entry.total_size();
            }
        }

        ret
    }

    fn from_lines<I: Iterator<Item = impl AsRef<str>>>(lines: &mut I) -> FsEntry {
        assert_eq!("$ cd /", lines.next().unwrap().as_ref());
        let mut new = FsEntry::with_name("/".to_owned());
        new.from_lines_inner(lines);
        return new;
    }

    fn from_lines_inner<I: Iterator<Item = impl AsRef<str>>>(&mut self, lines: &mut I) {
        assert_eq!("$ ls", lines.next().unwrap().as_ref());
        loop {
            match lines.next() {
                Some(line) => {
                    let s = line.as_ref();
                    if s.eq("$ cd ..") {
                        return;
                    }

                    if s.starts_with("$ cd") {
                        let name = s.rsplit_once(' ').unwrap().1;
                        let kid = self
                            .kids
                            .as_mut()
                            .unwrap()
                            .iter_mut()
                            .find(|k| k.name.as_str().eq(name))
                            .unwrap();
                        kid.from_lines_inner(lines);
                    } else if s.starts_with("dir ") {
                        let entry = FsEntry::with_name(s.strip_prefix("dir ").unwrap().to_owned());
                        if let Some(kids) = self.kids.as_mut() {
                            kids.push(entry);
                        } else {
                            self.kids = Some(vec![entry]);
                        }
                    } else {
                        let line_bytes = s.as_bytes();
                        let numbers = line_bytes
                            .iter()
                            .take_while(|b| **b < 0x40 && **b >= 0x30)
                            .count();
                        let size = unsafe { std::str::from_utf8_unchecked(&line_bytes[..numbers]) }
                            .parse()
                            .unwrap();
                        let name = s.split(' ').skip(1).next().unwrap();
                        let kid = FsEntry {
                            name: name.to_owned(),
                            size,
                            kids: None,
                        };
                        if let Some(kids) = self.kids.as_mut() {
                            kids.push(kid);
                        } else {
                            self.kids = Some(vec![kid]);
                        }
                    }
                }
                None => {
                    return;
                }
            }
        }
    }
}

impl AsRef<FsEntry> for FsEntry {
    fn as_ref(&self) -> &FsEntry {
        self
    }
}

impl AsMut<FsEntry> for FsEntry {
    fn as_mut(&mut self) -> &mut FsEntry {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_simple_tree() {
        let input = vec!["$ cd /", "$ ls", "dir a", "14848514 b.txt", "8504156 c.dat"];
        let expected = FsEntry {
            name: "/".to_owned(),
            size: 0,
            kids: Some(vec![
                FsEntry {
                    name: "a".to_owned(),
                    size: 0,
                    kids: None,
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
            ]),
        };

        assert_eq!(expected, FsEntry::from_lines(&mut input.iter()));
    }

    #[test]
    fn should_parse_simple_tree_with_subdir() {
        let input = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
        ];
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
                            kids: None,
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
            ]),
        };

        assert_eq!(expected, FsEntry::from_lines(&mut input.iter()));
    }

    #[test]
    fn should_parse_complex_tree() {
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

        let result = FsEntry::from_lines(&mut input.lines());
        assert_eq!(expected, result);
    }

    #[test]
    fn should_calc_the_right_answer() {
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

        assert_eq!(95437, p1(input));
    }

    #[test]
    fn should_calc_smallest_dir_to_delete() {
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

        assert_eq!(24933642, p2(input));
    }
}
