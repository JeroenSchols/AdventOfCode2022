mod star_1;
mod star_2;

fn main() {
    let input_file = "src/bin/day_8/input.txt";
    let heights: Vec<Vec<i8>> = std::fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| line.bytes()
            .map(|c| (c as char).to_string().parse::<i8>().unwrap())
            .collect()
        )
        .collect();

    let out_1 = star_1::run(&heights);
    println!("star 1: {out_1:?}");
    let out_2 = star_2::run(&heights);
    println!("star 2: {out_2:?}");
}
