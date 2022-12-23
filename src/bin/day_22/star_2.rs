use std::collections::HashMap;
use crate::{Entry, State};

pub fn create_graph(map: &Vec<Vec<Entry>>) -> HashMap<State, State> {
    let max_x = map.len();
    let max_y = map[0].len();
    let dim = max_x.max(max_y) / 4;
    assert!(dim > 0 && max_x % dim == 0 && max_y % dim == 0);

    let mut graph = HashMap::new();
    for y in 0..max_y {
        for x in 0..max_x - 1 {
            if map[x][y] != Entry::Empty && map[x + 1][y] != Entry::Empty {
                graph.insert(State { x, y, dir: 0 }, State { x: x + 1, y, dir: 0 });
                graph.insert(State { x: x + 1, y, dir: 2 }, State { x, y, dir: 2 });
            }
        }
    }

    for x in 0..max_x {
        for y in 0..max_y - 1 {
            if map[x][y] != Entry::Empty && map[x][y + 1] != Entry::Empty {
                graph.insert(State { x, y, dir: 1 }, State { x, y: y + 1, dir: 1 });
                graph.insert(State { x, y: y + 1, dir: 3 }, State { x, y, dir: 3 });
            }
        }
    }

    for i in 0..dim {
        // hardcoded for the cube layout in input.txt
        graph.insert(State { x: dim + i, y: 0, dir: 3 }, State { x: 0, y: 3 * dim + i, dir: 0 });
        graph.insert(State { x: 2 * dim + i, y: 0, dir: 3 }, State { x: i, y: 4 * dim - 1, dir: 3 });
        graph.insert(State { x: 2 * dim + i, y: dim - 1, dir: 1 }, State { x: 2 * dim - 1, y: dim + i, dir: 2 });
        graph.insert(State { x: i, y: 2 * dim, dir: 3 }, State { x: dim, y: dim + i, dir: 0 });
        graph.insert(State { x: dim + i, y: 3 * dim - 1, dir: 1 }, State { x: dim - 1, y: 3 * dim + i, dir: 2 });
        graph.insert(State { x: i, y: 4 * dim - 1, dir: 1 }, State { x: 2 * dim + i, y: 0, dir: 1 });
        graph.insert(State { x: dim, y: i, dir: 2 }, State { x: 0, y: 3 * dim - 1 - i, dir: 0 });
        graph.insert(State { x: 3 * dim - 1, y: i, dir: 0 }, State { x: 2 * dim - 1, y: 3 * dim - 1 - i, dir: 2 });
        graph.insert(State { x: dim, y: dim + i, dir: 2 }, State { x: i, y: 2 * dim, dir: 1 });
        graph.insert(State { x: 2 * dim - 1, y: dim + i, dir: 0 }, State { x: 2 * dim + i, y: dim - 1, dir: 3 });
        graph.insert(State { x: 0, y: 2 * dim + i, dir: 2 }, State { x: dim, y: dim - 1 - i, dir: 0 });
        graph.insert(State { x: 2 * dim - 1, y: 2 * dim + i, dir: 0 }, State { x: 3 * dim - 1, y: dim - 1 - i, dir: 2 });
        graph.insert(State { x: 0, y: 3 * dim + i, dir: 2 }, State { x: dim + i, y: 0, dir: 1 });
        graph.insert(State { x: dim - 1, y: 3 * dim + i, dir: 0 }, State { x: dim + i, y: 3 * dim - 1, dir: 3 });
    }

    graph
}
