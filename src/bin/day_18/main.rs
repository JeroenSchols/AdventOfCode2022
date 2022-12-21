mod star_1;
mod star_2;

use std::collections::HashSet;

fn parse_input(file: &str) -> HashSet<(i8, i8, i8)> {
    std::fs::read_to_string(file).unwrap()
        .lines().map(|line| {
        let mut it = line.split(',').map(|val| val.parse::<i8>().unwrap());
        (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
    }).collect::<HashSet<(i8, i8, i8)>>()
}

fn main() {
    let input_file = "src/bin/day_18/input.txt";
    let pixels = parse_input(input_file);
    let out_1 = star_1::run(&pixels);
    println!("star 1: {out_1}");
    let out_2 = star_2::run(&pixels);
    println!("star 2: {out_2}");
}
