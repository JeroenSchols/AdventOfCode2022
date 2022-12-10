use regex::{Match, Regex};

mod star_1;
mod star_2;

fn parse_re_capture_int(capture: Option<Match>) -> usize {
    capture.unwrap().as_str().parse::<usize>().unwrap()
}

fn main() {
    let input_file = "src/bin/day_5/input.txt";
    let input_str = std::fs::read_to_string(input_file).unwrap();
    let (init_state_str, moves_str) = input_str.split_once("\r\n\r\n").unwrap();
    let num_stacks = init_state_str.find("\r\n").unwrap() / 4 + 1;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];
    init_state_str.lines()
        .rev()
        .skip(1)
        .for_each(|level| level.bytes()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(idx, c)|
                if c as char != ' ' {
                    stacks[idx].push(c as char);
                }
            )
        );
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let moves: Vec<(usize, usize, usize)> = moves_str.lines()
        .map(|line| {
            let caps = re.captures(line).expect("regex matches line");
            (
                parse_re_capture_int(caps.get(1)),
                parse_re_capture_int(caps.get(2)) - 1,
                parse_re_capture_int(caps.get(3)) - 1
            )
        })
        .collect();

    let out_1 = star_1::run(&stacks, &moves);
    println!("star 1: {out_1:?}");
    let out_2 = star_2::run(&stacks, &moves);
    println!("star 2: {out_2:?}");
}
