use std::io::Read;

fn main() {
    let input = read_input();
    let mut school_of_fish = SoF::new();

    for b in input {
        school_of_fish.add_fish(b);
    }
    dbg!(&school_of_fish);

    for _ in 0..256 {
        school_of_fish.age();
    }

    println!("{}", school_of_fish.count());
}

fn read_input() -> Vec<u8> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[derive(Debug)]
struct SoF {
    groups: [Group; 7],
    new_groups: [NewGroup; 2],
    /// Points towards the group which takes the new ones
    ptr: usize,
}

impl SoF {
    pub fn new() -> SoF {
        SoF {
            groups: [
                Group(0, 0),
                Group(1, 0),
                Group(2, 0),
                Group(3, 0),
                Group(4, 0),
                Group(5, 0),
                Group(6, 0),
            ],
            new_groups: [NewGroup(0, 0), NewGroup(1, 0)],
            ptr: 6,
        }
    }

    pub fn add_fish(&mut self, fish: u8) {
        match fish {
            0..=6 => self.groups[fish as usize].1 += 1,
            7 | 8 => self.new_groups[(fish - 7) as usize].1 += 1,
            _ => panic!("Invalid data."),
        }
    }

    pub fn age(&mut self) {
        let mut to_add = 0;
        for g in self.groups.iter_mut() {
            if let Some(new_fish) = g.age() {
                to_add = new_fish;
            }
        }

        match self.ptr.cmp(&6) {
            std::cmp::Ordering::Equal => self.ptr = 0,
            _ => self.ptr += 1,
        }

        self.groups[self.ptr].1 += self.new_groups[0].1;
        self.new_groups[0].1 = self.new_groups[1].1;
        self.new_groups[1].1 = to_add;
    }

    pub fn count(&self) -> u64 {
        self.groups.iter().fold(0, |cnt, g| cnt + g.1)
            + self.new_groups.iter().fold(0, |cnt, g| cnt + g.1)
    }
}

#[derive(Debug)]
struct NewGroup(u8, u64);

#[derive(Debug)]
struct Group(u8, u64);

impl Group {
    pub fn age(&mut self) -> Option<u64> {
        if self.0 == 0 {
            self.0 = 6;
            Some(self.1)
        } else {
            self.0 -= 1;
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aging() {
        let mut school = SoF::new();
        school.add_fish(1);
        school.add_fish(2);
        school.add_fish(3);
        school.add_fish(3);
        school.add_fish(4);
        school.add_fish(5);
        school.add_fish(6);
        school.add_fish(6);
        school.add_fish(0);
        school.add_fish(7);
        school.add_fish(7);

        school.age();

        assert_eq!(school.ptr, 0);
        assert_eq!(school.count(), 12);
        assert_eq!(school.new_groups[0].1, 0);
        assert_eq!(school.new_groups[1].1, 1);
        assert_eq!(school.groups[0].1, 3);
    }
}
