fn parse(input_file: &str) -> ((usize, usize), (usize, usize), Vec<Vec<u8>>) {
    let input = std::fs::read_to_string(input_file).unwrap();
    let mut heights = Vec::new();
    let mut start: Result<(usize, usize), ()> = Err(());
    let mut end: Result<(usize, usize), ()> = Err(());
    for (x, line) in input.lines().enumerate() {
        heights.push(Vec::new());
        for (y, c) in line.bytes().enumerate() {
            heights.last_mut().unwrap().push(match c {
                b'S' => { start = Ok((x, y)); 0 }
                b'E' => { end = Ok((x, y)); 25 }
                x => x - b'a'
            });
        }
    }
    assert!(start.is_ok());
    assert!(end.is_ok());
    (start.ok().unwrap(), end.ok().unwrap(), heights)
}

fn bfs(heights: &Vec<Vec<u8>>, end: &(usize, usize), mut cur: Vec<(usize, usize)>) -> u32 {
    let mut visited = vec![vec![false; heights[0].len()]; heights.len()];
    let mut next = Vec::new();
    let moves = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    for depth in 0.. {
        for (x, y) in &cur {
            if (*x, *y) == *end {
                return depth;
            }
            for (dx, dy) in &moves {
                let nx = (*x as isize + dx) as usize;
                let ny = (*y as isize + dy) as usize;
                if nx < visited.len() && ny < visited[nx].len() && !visited[nx][ny] && heights[nx][ny] <= heights[*x][*y] + 1 {
                    visited[nx][ny] = true;
                    next.push((nx, ny));
                }
            }
        }
        std::mem::swap(&mut cur, &mut next);
    }
    unreachable!();
}

fn main() {
    let input_file = "src/bin/day_12/input.txt";
    let (start, end, heights) = parse(input_file);

    let out_1 = bfs(&heights, &end, vec![start]);
    println!("star 1: {out_1:?}");

    let mut in_2 = Vec::new();
    for (x, row) in heights.iter().enumerate() {
        for (y, height) in row.iter().enumerate() {
            if *height == 0 {
                in_2.push((x, y));
            }
        }
    }
    let out_2 = bfs(&heights, &end, in_2);
    println!("star 2: {out_2:?}");
}
