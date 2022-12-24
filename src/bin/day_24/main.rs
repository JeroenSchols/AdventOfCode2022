use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let file = "src/bin/day_24/input.txt";
    let input = std::fs::read_to_string(file).unwrap().lines()
        .map(|line| line.bytes().collect_vec()).collect_vec();
    let max_x = input[0].len() + 2;
    let max_y = input.len() + 2;
    let mut block = vec![vec![vec![false; max_y]; max_x]; 5];
    for (y, row) in input.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            match v {
                b'#' => block[0][x + 1][y + 1] = true,
                b'>' => block[1][x + 1][y + 1] = true,
                b'v' => block[2][x + 1][y + 1] = true,
                b'<' => block[3][x + 1][y + 1] = true,
                b'^' => block[4][x + 1][y + 1] = true,
                _ => {}
            }
        }
    }
    for x in 0..max_x {
        block[0][x][0] = true;
        block[0][x][max_y - 1] = true;
    }
    for y in 0..max_y {
        block[0][0][y] = true;
        block[0][max_x - 1][y] = true;
    }

    let mut out_1 = u16::MAX;
    let mut out_2 = u16::MAX;
    let moves = vec![(0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)];
    let start = (2, 1);
    let end = (max_x - 3, max_y - 2);
    let mut state = HashSet::new();
    state.insert((start.0, start.1, 0));
    'outer: for i in 0.. {
        for x in 2..max_x - 2 {
            let mut prev = block[2][x][max_y - 3];
            for y in 2..max_y - 2 {
                (block[2][x][y], prev) = (prev, block[2][x][y]);
            }
            prev = block[4][x][2];
            for y in (2..max_y - 2).rev() {
                (block[4][x][y], prev) = (prev, block[4][x][y]);
            }
        }
        for y in 2..max_y - 2 {
            let mut prev = block[1][max_x - 3][y];
            for x in 2..max_x - 2 {
                (block[1][x][y], prev) = (prev, block[1][x][y]);
            }
            prev = block[3][2][y];
            for x in (2..max_x - 2).rev() {
                (block[3][x][y], prev) = (prev, block[3][x][y]);
            }
        }

        let mut next_state = HashSet::new();
        for (x, y, s) in &state {
            let mut ns = s;
            if *s == 0 && (*x, *y) == end {
                out_1 = out_1.min(i);
                ns = &1;
            } else if *s == 1 && (*x, *y) == start {
                ns = &2;
            } else if *s == 2 && (*x, *y) == end {
                out_2 = out_2.min(i);
                break 'outer;
            }
            for (dx, dy) in &moves {
                let nx = (*x as isize + dx) as usize;
                let ny = (*y as isize + dy) as usize;
                if block.iter().map(|b| b[nx][ny] as u8).sum::<u8>() == 0 {
                    next_state.insert((nx, ny, *ns));
                }
            }
        }
        state = next_state;
    }

    println!("star 1: {out_1}");
    println!("star 2: {out_2}");
}



