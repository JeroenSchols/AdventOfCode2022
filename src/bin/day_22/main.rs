mod star_1;
mod star_2;

use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
pub enum Entry {
    Empty,
    Open,
    Blocked,
}

enum MoveType {
    Forward,
    Rotate,
}

struct Move {
    move_type: MoveType,
    by: usize,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct State {
    x: usize,
    y: usize,
    dir: usize,
}

fn parse_input(input_file: &str) -> (Vec<Vec<Entry>>, Vec<Move>) {
    let input = std::fs::read_to_string(input_file).unwrap();
    let mut split_input = input.split("\r\n\r\n");
    let input_map = split_input.next().unwrap();
    let input_movement = split_input.next().unwrap();

    let max_x = input_map.lines().map(|line| line.len()).max().unwrap();
    let max_y = input_map.lines().count();
    let mut map = vec![vec![Entry::Empty; max_y]; max_x];
    for (y, line) in input_map.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            if c == b'.' {
                map[x][y] = Entry::Open;
            } else if c == b'#' {
                map[x][y] = Entry::Blocked;
            }
        }
    }

    let mut moves = Vec::new();
    let mut by = 0;
    for c in (input_movement.lines().next().unwrap().to_string() + "|").bytes() {
        if b'0' <= c && c <= b'9' {
            by = by * 10 + c - b'0';
        } else {
            moves.push(Move {
                move_type: MoveType::Forward,
                by: by as usize,
            });
            by = 0;
        }
        if c == b'L' {
            moves.push(Move {
                move_type: MoveType::Rotate,
                by: 3,
            });
        } else if c == b'R' {
            moves.push(Move {
                move_type: MoveType::Rotate,
                by: 1,
            });
        }
    }

    (map, moves)
}

fn remove_blocked_entries(map: &Vec<Vec<Entry>>, graph: &mut HashMap<State, State>) {
    for x1 in 0..map.len() {
        for y1 in 0..map[0].len() {
            if map[x1][y1] == Entry::Blocked {
                for dir1 in 0..4 {
                    if let Some(State { x: x2, y: y2, dir: dir2 }) = graph.get(&State { x: x1, y: y1, dir: dir1 }) {
                        graph.remove(&State { x: *x2, y: *y2, dir: (dir2 + 2) % 4 });
                        graph.remove(&State { x: x1, y: y1, dir: dir1 });
                    }
                }
            }
        }
    }
}

fn simulate(graph: &HashMap<State, State>, mut state: State, moves: &Vec<Move>) -> State {
    for m in moves {
        match m.move_type {
            MoveType::Forward => for _ in 0..m.by {
                if let Some(next_state) = graph.get(&state) {
                    state = next_state.clone();
                }
            }
            MoveType::Rotate => state.dir = (state.dir + m.by) % 4
        }
    }
    state
}

fn main() {
    let file = "src/bin/day_22/input.txt";
    let (map, moves) = parse_input(file);
    let start_state = State {
        x: map.iter().position(|col| col[0] == Entry::Open).unwrap(),
        y: 0,
        dir: 0,
    };

    let mut graph_1 = star_1::create_graph(&map);
    remove_blocked_entries(&map, &mut graph_1);
    let end_state_1 = simulate(&graph_1, start_state.clone(), &moves);
    let out_1 = 4 * end_state_1.x + 1000 * end_state_1.y + end_state_1.dir + 1004;
    println!("star 1: {out_1}");

    let mut graph_2 = star_2::create_graph(&map);
    remove_blocked_entries(&map, &mut graph_2);
    let end_state_2 = simulate(&graph_2, start_state.clone(), &moves);
    let out_2 = 4 * end_state_2.x + 1000 * end_state_2.y + end_state_2.dir + 1004;
    println!("star 2: {out_2}");
}
