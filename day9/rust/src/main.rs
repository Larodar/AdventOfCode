fn main() {
    println!("Hello, world!");
}

fn search_min() -> Option<(u16, u16)> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_local_min() {
        let input = concat!(
            "2199943210\n",
            "3987894921\n",
            "9856789892\n",
            "8767896789\n",
            "9899965678\n"
        );

        assert_eq!(search_min(), Some((0, 1)));
    }
}
