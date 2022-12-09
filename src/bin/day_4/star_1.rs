pub fn run(parsed_input: &[(u64, u64, u64, u64)]) -> u64 {
    parsed_input.iter()
        .filter(|(a, b, c, d)| b >= c && d >= a)
        .count() as u64
}
