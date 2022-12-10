pub fn run(heights: &Vec<Vec<i8>>) ->u64 {
    let max_x = heights.len();
    let max_y = heights[0].len();
    let mut best = 0u64;

    for ix in 1..max_x-1 {
        for iy in 1..max_y-1 {
            let mut score: u64 = 1;
            let mut vis: u64 = 0;
            for y in (0..iy).rev() {
                vis += 1;
                if heights[ix][y] >= heights[ix][iy] {
                    break;
                }
            }
            score *= vis;
            vis = 0;
            for y in iy+1..max_y {
                vis += 1;
                if heights[ix][y] >= heights[ix][iy] {
                    break;
                }
            }
            score *= vis;
            vis = 0;
            for x in (0..ix).rev() {
                vis += 1;
                if heights[x][iy] >= heights[ix][iy] {
                    break;
                }
            }
            score *= vis;
            vis = 0;
            for x in ix+1..max_x {
                vis += 1;
                if heights[x][iy] >= heights[ix][iy] {
                    break;
                }
            }
            score *= vis;
            best = best.max(score);
        }
    }

    best
}
