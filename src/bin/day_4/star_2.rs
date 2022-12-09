pub fn run(parsed_input: &[(u64, u64, u64, u64)]) -> u64 {
    parsed_input.iter()
        .filter(|(a, b, c, d)| (a <= c && d <= b) || (c <= a && b <= d))
        .count() as u64
}
