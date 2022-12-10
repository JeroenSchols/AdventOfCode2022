pub fn run(heights: &Vec<Vec<i8>>) -> usize {
    let max_x = heights.len();
    let max_y = heights[0].len();
    let mut visible: Vec<Vec<bool>> = vec![vec![false; max_y]; max_x];

    for x in 0..max_x {
        let mut height = -1i8;
        for y in 0..max_y {
            if heights[x][y] > height {
                visible[x][y] = true;
                height = heights[x][y];
            }
        }
        height = -1i8;
        for y in (0..max_y).rev() {
            if heights[x][y] > height {
                visible[x][y] = true;
                height = heights[x][y];
            }
        }
    }
    for y in 0..max_y {
        let mut height = -1i8;
        for x in 0..max_x {
            if heights[x][y] > height {
                visible[x][y] = true;
                height = heights[x][y];
            }
        }
        height = -1i8;
        for x in (0..max_x).rev() {
            if heights[x][y] > height {
                visible[x][y] = true;
                height = heights[x][y];
            }
        }
    }

    visible.iter().map(|col| col.iter().filter(|vis| **vis).count()).sum()
}
