pub fn run(input_file: &str) -> u64 {
    std::fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| match line {
            "A X" => Ok(4),
            "A Y" => Ok(8),
            "A Z" => Ok(3),
            "B X" => Ok(1),
            "B Y" => Ok(5),
            "B Z" => Ok(9),
            "C X" => Ok(7),
            "C Y" => Ok(2),
            "C Z" => Ok(6),
            _ => Err("invalid turn")
        }.unwrap())
        .sum()
}
