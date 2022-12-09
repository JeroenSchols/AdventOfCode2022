pub fn run(input_file: &str) -> u64 {
    std::fs::read_to_string(input_file)
        .unwrap()
        .split("\r\n\r\n")
        .map(|elf| elf.lines().map(|str| str.parse::<u64>().unwrap()).sum())
        .max()
        .unwrap()
}
