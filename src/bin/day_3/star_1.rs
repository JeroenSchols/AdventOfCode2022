use std::collections::HashSet;

pub fn run(input_file: &str) -> u64 {
    std::fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let left_charset = left.bytes().collect::<HashSet<_>>();
            let right_charset = right.bytes().collect::<HashSet<_>>();
            let mut intersection = left_charset.intersection(&right_charset);
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
