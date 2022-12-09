use std::collections::HashSet;
use itertools::Itertools;

pub fn run(input_file: &str) -> u64 {
    std::fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| line.bytes().collect::<HashSet<_>>())
        .tuples()
        .map(|(bag1, bag2, bag3)| {
            let mut intersection = bag1.iter().filter(|char| bag2.contains(char) && bag3.contains(char));
            let char = intersection.next().expect("non empty intersection");
            assert!(intersection.next().is_none(), "should have unique char in intersection");
            match char {
                b'a'..=b'z' => Ok((char - b'a' + 1) as u64),
                b'A'..=b'Z' => Ok((char - b'A' + 27) as u64),
                _ => Err("invalid char")
            }.unwrap()
        })
        .sum()
}
