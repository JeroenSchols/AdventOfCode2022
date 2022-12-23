use std::collections::HashMap;
use crate::{Entry, State};

pub fn create_graph(map: &Vec<Vec<Entry>>) -> HashMap<State, State> {
    let max_x = map.len();
    let max_y = map[0].len();
    let mut graph = HashMap::new();

    for y in 0..max_y {
        let mut prev_x = Option::<usize>::None;
        let mut first_x = max_x;
        let mut last_x = 0;
        for x1 in 0..max_x {
            if map[x1][y] != Entry::Empty {
                if let Some(x2) = prev_x {
                    graph.insert(State { x: x1, y, dir: 2 }, State { x: x2, y, dir: 2 });
                    graph.insert(State { x: x2, y, dir: 0 }, State { x: x1, y, dir: 0 });
                }
                first_x = first_x.min(x1);
                last_x = last_x.max(x1);
                prev_x = Some(x1);
            }
        }
        graph.insert(State { x: first_x, y, dir: 2 }, State { x: last_x, y, dir: 2 });
        graph.insert(State { x: last_x, y, dir: 0 }, State { x: first_x, y, dir: 0 });
    }

    for x in 0..max_x {
        let mut prev_y = Option::<usize>::None;
        let mut first_y = max_y;
        let mut last_y = 0;
        for y1 in 0..max_y {
            if map[x][y1] != Entry::Empty {
                if let Some(y2) = prev_y {
                    graph.insert(State { x, y: y1, dir: 3 }, State { x, y: y2, dir: 3 });
                    graph.insert(State { x, y: y2, dir: 1 }, State { x, y: y1, dir: 1 });
                }
                first_y = first_y.min(y1);
                last_y = last_y.max(y1);
                prev_y = Some(y1);
            }
        }
        graph.insert(State { x, y: first_y, dir: 3 }, State { x, y: last_y, dir: 3 });
        graph.insert(State { x, y: last_y, dir: 1 }, State { x, y: first_y, dir: 1 });
    }

    graph
}
