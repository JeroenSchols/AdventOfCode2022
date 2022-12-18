use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

fn parse_input(input_file: &str) -> (Vec<Vec<u64>>, Vec<u64>, usize) {
    let re = Regex::new(r"^Valve (.+) has flow rate=(.+); tunnels? leads? to valves? (.+)$").unwrap();
    let input = std::fs::read_to_string(input_file).unwrap()
        .lines().map(|line| {
        let caps = re.captures(line).expect("regex matches line");
        let name = caps.get(1).unwrap().as_str().to_string();
        let weight = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
        let neighbours = caps.get(3).unwrap().as_str().split(", ").map(|str| str.to_string()).collect_vec();
        (name, weight, neighbours)
    }).collect_vec();

    let name_to_idx = input.iter().enumerate()
        .map(|(idx, (name, _val, _neighbours))| (name.to_string(), idx))
        .collect::<HashMap<String, usize>>();

    let relevant_indices = input.iter().enumerate()
        .filter_map(|(idx, (name, weight, _neighbours))|
            if *weight > 0 || name == "AA" {
                Some(idx)
            } else {
                None
            }
        ).collect_vec();
    let relevant_flow_rates = relevant_indices.iter().map(|idx| input[*idx].1).collect_vec();

    let mut distances = vec![vec![31; name_to_idx.len()]; name_to_idx.len()];
    for (from, _weight, neighbours) in &input {
        for to in neighbours {
            distances[*name_to_idx.get(from).unwrap()][*name_to_idx.get(to).unwrap()] = 1;
        }
    }
    for _ in 0..name_to_idx.len() {
        for from in 0..name_to_idx.len() {
            for to in 0..name_to_idx.len() {
                for via in 0..name_to_idx.len() {
                    distances[from][to] = distances[from][to].min(distances[from][via] + distances[via][to]);
                }
            }
        }
    }

    let relevant_distances = relevant_indices.iter().map(|from| relevant_indices.iter().map(|to| distances[*from][*to]).collect_vec()).collect_vec();
    let start_idx = input.iter().find_position(|(name, _weight, _neighbours)| name == "AA").unwrap().0;
    let relevant_start_idx = relevant_indices.iter().find_position(|idx| **idx == start_idx).unwrap().0;

    (relevant_distances, relevant_flow_rates, relevant_start_idx)
}

fn rec(distances: &Vec<Vec<u64>>, flow_rates: &Vec<u64>, cache: &mut HashMap<(u64, usize, u64), u64>, mask: u64, cur_idx: usize, cur_time: u64) -> u64 {
    if let Some(val) = cache.get(&(mask, cur_idx, cur_time)) {
        return *val;
    }
    let mut best = 0;
    for (next_idx, dist) in distances[cur_idx].iter().enumerate() {
        if mask & (1 << next_idx) == 0 && cur_time + dist < 30 {
            best = best.max(rec(
                distances,
                flow_rates,
                cache,
                mask + (1 << next_idx),
                next_idx,
                cur_time + dist + 1,
            ) + (30 - cur_time - dist - 1) * flow_rates[next_idx]);
        }
    }
    cache.insert((mask, cur_idx, cur_time), best);
    best
}

fn star_1(distances: &Vec<Vec<u64>>, flow_rates: &Vec<u64>, start_idx: usize) -> u64 {
    let mut cache = HashMap::new();
    rec(distances, flow_rates, &mut cache, 0, start_idx, 0)
}

fn star_2(distances: &Vec<Vec<u64>>, flow_rates: &Vec<u64>, start_idx: usize) -> u64 {
    let mut mem = HashMap::new();
    let mut best = 0;
    for i in 0..1 << (flow_rates.len() - 1) {
        let j = (1 << flow_rates.len()) - 1 ^ i;
        best = best.max(
            rec(distances, flow_rates, &mut mem, i, start_idx, 4) +
            rec(distances, flow_rates, &mut mem, j, start_idx, 4)
        );
    }
    best
}

fn main() {
    let file = "src/bin/day_16/input.txt";
    let (edges, flow_rates, start_idx) = parse_input(file);
    let out_1 = star_1(&edges, &flow_rates, start_idx);
    println!("star 1: {out_1}");
    let out_2 = star_2(&edges, &flow_rates, start_idx);
    println!("star 2: {out_2}");
}
