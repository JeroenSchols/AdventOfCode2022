use std::collections::HashSet;

pub fn run(input_file: &str, window_size: usize) -> usize {
    std::fs::read_to_string(input_file)
        .unwrap()
        .as_bytes()
        .windows(window_size)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == window_size)
        .unwrap() + window_size
}

fn main() {
    let file = "src/bin/day_6/input.txt";
    let out_1 = run(file, 4);
    println!("star 1: {out_1:?}");
    let out_2 = run(file, 14);
    println!("star 2: {out_2:?}");
}
