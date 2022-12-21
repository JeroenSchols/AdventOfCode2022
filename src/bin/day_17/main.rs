use std::collections::{HashMap, VecDeque};
use itertools::Itertools;

#[derive(PartialEq)]
enum Dir {
    Left,
    Right,
    Down,
}

fn parse_input(input_file: &str) -> Vec<Dir> {
    std::fs::read_to_string(input_file).unwrap()
        .bytes().flat_map(|c| if c == b'>' {
        vec![Dir::Right, Dir::Down]
    } else if c == b'<' {
        vec![Dir::Left, Dir::Down]
    } else {
        vec![]
    }).collect_vec()
}

fn get_rocks() -> Vec<Vec<i32>> {
    vec![
        vec![
            0b000000000,
            0b000000000,
            0b000000000,
            0b000111100,
        ],
        vec![
            0b000000000,
            0b000010000,
            0b000111000,
            0b000010000,
        ],
        vec![
            0b000000000,
            0b000001000,
            0b000001000,
            0b000111000,
        ],
        vec![
            0b000100000,
            0b000100000,
            0b000100000,
            0b000100000,
        ],
        vec![
            0b000000000,
            0b000000000,
            0b000110000,
            0b000110000,
        ],
    ]
}

fn state_to_string(state: &VecDeque<i32>) -> String {
    let mut string = String::new();
    for row in state {
        string.push_str(row.to_string().as_str());
        string.push(',');
    }
    string
}

fn simulate(rocks: &Vec<Vec<i32>>, moves: &Vec<Dir>, n: u64) -> u64 {
    let mut cache: HashMap<(String, usize, usize), (u64, u64)> = HashMap::new();
    let mut state: VecDeque<i32> = VecDeque::from(vec![0b100000001; 50]);
    state[7] = 0b111111111;

    let mut iteration = 0;
    let mut height = 0;
    let mut rock_idx = rocks.len() - 1;
    let mut move_idx = moves.len() - 1;


    while iteration < n {
        if let Some((prev_iteration, prev_height)) = cache.get(&(state_to_string(&state), rock_idx, move_idx)) {
            let delta_iteration = iteration - prev_iteration;
            let delta_height = height - prev_height;
            let steps = (n - iteration) / delta_iteration;
            iteration += delta_iteration * steps;
            height += delta_height * steps;
        } else {
            cache.insert((state_to_string(&state), rock_idx, move_idx), (iteration, height));
        }
        rock_idx = (rock_idx + 1) % rocks.len();
        let mut rock: VecDeque<i32> = VecDeque::from(rocks[rock_idx].clone());
        loop {
            move_idx = (move_idx + 1) % moves.len();
            let next_rock = match moves[move_idx] {
                Dir::Left => rock.iter().map(|row| *row << 1).collect::<VecDeque<i32>>(),
                Dir::Right => rock.iter().map(|row| *row >> 1).collect::<VecDeque<i32>>(),
                Dir::Down => {
                    let mut new_rock = rock.clone();
                    new_rock.push_front(0);
                    new_rock
                }
            };
            let mut valid = true;
            for idx in 0..next_rock.len() {
                if next_rock[idx] & state[idx] != 0 {
                    valid = false;
                }
            }
            if valid {
                rock = next_rock;
            } else if moves[move_idx] == Dir::Down {
                for idx in 0..rock.len() {
                    state[idx] += rock[idx];
                }
                break;
            }
        }
        let mut top_index = 0;
        while state[top_index] == 0b100000001 {
            top_index += 1;
        }
        while top_index < 7 {
            top_index += 1;
            height += 1;
            state.push_front(0b100000001);
            state.pop_back();
        }
        iteration += 1;
    }
    height
}

fn main() {
    let input_file = "src/bin/day_17/input.txt";
    let moves = parse_input(input_file);
    let rocks = get_rocks();
    let out_1 = simulate(&rocks, &moves, 2022);
    println!("star 1: {out_1:?}");
    let out_2 = simulate(&rocks, &moves, 1000000000000);
    println!("star 2: {out_2:?}");
}
