use std::collections::{HashMap, HashSet};

fn parse_moves(input_file: &str) -> Vec<(i32, i32)> {
    std::fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .flat_map(|line| {
            let mut iter = line.split_whitespace();
            let dir = iter.next().unwrap().parse::<char>().unwrap();
            let count = iter.next().unwrap().parse::<usize>().unwrap();
            let delta = match dir {
                'R' => Ok((1, 0)),
                'L' => Ok((-1, 0)),
                'U' => Ok((0, 1)),
                'D' => Ok((0, -1)),
                _ => Err("invalid direction")
            }.unwrap();
            vec![delta; count]
        })
        .collect()
}

fn simulate(moves: &Vec<(i32, i32)>, num_knots: usize) -> usize {
    let move_table: HashMap<(i32, i32), (i32, i32)> = HashMap::from([
        ((-2,-2),(-1,-1)),((-1,-2),(-1,-1)),(( 0,-2),( 0,-1)),(( 1,-2),( 1,-1)),(( 2,-2),( 1,-1)),
        ((-2,-1),(-1,-1)),((-1,-1),( 0, 0)),(( 0,-1),( 0, 0)),(( 1,-1),( 0, 0)),(( 2,-1),( 1,-1)),
        ((-2, 0),(-1, 0)),((-1, 0),( 0, 0)),(( 0, 0),( 0, 0)),(( 1, 0),( 0, 0)),(( 2, 0),( 1, 0)),
        ((-2, 1),(-1, 1)),((-1, 1),( 0, 0)),(( 0, 1),( 0, 0)),(( 1, 1),( 0, 0)),(( 2, 1),( 1, 1)),
        ((-2, 2),(-1, 1)),((-1, 2),(-1, 1)),(( 0, 2),( 0, 1)),(( 1, 2),( 1, 1)),(( 2, 2),( 1, 1)),
    ]);
    let mut pos: Vec<(i32, i32)> = vec![(0,0); num_knots];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for (dhx, dhy) in moves {
        pos[0].0 += dhx;
        pos[0].1 += dhy;
        for idx in 1..pos.len() {
            let (dx, dy) = move_table[&(pos[idx-1].0 - pos[idx].0, pos[idx-1].1 - pos[idx].1)];
            pos[idx].0 += dx;
            pos[idx].1 += dy;
        }
        visited.insert(*pos.last().unwrap());
    }
    visited.len()
}

fn main() {
    let moves = parse_moves("src/bin/day_9/input.txt");
    let out_1 = simulate(&moves, 2);
    println!("star 1: {out_1}");
    let out_2 = simulate(&moves, 10);
    println!("star 2: {out_2}");
}
