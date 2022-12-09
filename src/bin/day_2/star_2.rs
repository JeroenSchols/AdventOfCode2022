pub fn run(input_file: &str) -> u64 {
    std::fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| match line {
            "A X" => Ok(3),
            "A Y" => Ok(4),
            "A Z" => Ok(8),
            "B X" => Ok(1),
            "B Y" => Ok(5),
            "B Z" => Ok(9),
            "C X" => Ok(2),
            "C Y" => Ok(6),
            "C Z" => Ok(7),
            _ => Err("invalid turn")
        }.unwrap())
        .sum()
}
