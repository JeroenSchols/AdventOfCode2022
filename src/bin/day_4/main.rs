use regex::{Match, Regex};

mod star_1;
mod star_2;

fn parse_re_capture_int(capture: Option<Match>) -> u64 {
    capture.unwrap().as_str().parse::<u64>().unwrap()
}

fn main() {
    let input_file = "src/bin/day_4/input.txt";

    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let parsed_input: Vec<(u64, u64, u64, u64)> = std::fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|line| {
            let caps = re.captures(line).expect("regex matches line");
            (
                parse_re_capture_int(caps.get(1)),
                parse_re_capture_int(caps.get(2)),
                parse_re_capture_int(caps.get(3)),
                parse_re_capture_int(caps.get(4))
            )
        })
        .collect();

    let out_1 = star_1::run(&parsed_input);
    println!("star 1: {out_1:?}");
    let out_2 = star_2::run(&parsed_input);
    println!("star 2: {out_2:?}");
}
