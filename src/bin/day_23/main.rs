use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};

fn main() {
    let file = "src/bin/day_23/input.txt";
    let mut state = HashSet::new();
    for (y, row) in std::fs::read_to_string(file).unwrap().lines().enumerate() {
        for (x, v) in row.bytes().enumerate() {
            if v == b'#' {
                state.insert((x as i16, y as i16));
            }
        }
    }

    let adj = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let moves = vec![
        (0, -1, vec![(-1, -1), (0, -1), (1, -1)]),
        (0, 1, vec![(-1, 1), (0, 1), (1, 1)]),
        (-1, 0, vec![(-1, -1), (-1, 0), (-1, 1)]),
        (1, 0, vec![(1, -1), (1, 0), (1, 1)]),
    ];

    let mut m = 0;
    loop {
        if m == 10 {
            let mut min_x = i16::MAX;
            let mut max_x = i16::MIN;
            let mut min_y = i16::MAX;
            let mut max_y = i16::MIN;
            for (x, y) in &state {
                min_x = min_x.min(*x);
                max_x = max_x.max(*x);
                min_y = min_y.min(*y);
                max_y = max_y.max(*y);
            }
            let out_1 = (max_x - min_x + 1) * (max_y - min_y + 1) - state.len() as i16;
            println!("star 1: {out_1}");
        }

        let mut propose_count = HashMap::new();
        let mut next_state = state.iter().map(|(x, y)| ((*x, *y), (*x, *y))).collect::<HashMap<(i16, i16), (i16, i16)>>();
        for ((x, y), (nx, ny)) in &mut next_state {
            if adj.iter().filter(|(dx, dy)| state.contains(&(x + dx, y + dy))).count() > 0 {
                for dm in 0..4 {
                    let (dx, dy, block) = &moves[(m + dm) % 4];
                    if block.iter().filter(|(ax, ay)| state.contains(&(x + ax, y + ay))).count() == 0 {
                        (*nx, *ny) = (x + dx, y + dy);
                        break;
                    }
                }
            }
            *propose_count.entry((*nx, *ny)).or_insert(0).borrow_mut() += 1;
        }

        let mut finished = true;
        for ((x, y), (nx, ny)) in next_state {
            if (x != nx || y != ny) && propose_count[&(nx, ny)] == 1 {
                state.remove(&(x, y));
                state.insert((nx, ny));
                finished = false;
            }
        }

        m += 1;
        if finished {
            println!("star 2: {m}");
            break;
        }
    }
}
