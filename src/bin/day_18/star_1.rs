use std::collections::HashSet;

pub fn run(pixels: &HashSet<(i8, i8, i8)>) -> usize {
    let adj: Vec<(i8, i8, i8)> = vec![(-1, 0, 0), (0, -1, 0), (0, 0, -1), (0, 0, 1), (0, 1, 0), (1, 0, 0)];
    let mut adj_count = 0;
    for (x, y, z) in pixels {
        for (dx, dy, dz) in &adj {
            if pixels.contains(&(x + dx, y + dy, z + dz)) {
                adj_count += 1;
            }
        }
    }
    6 * pixels.len() - adj_count
}
