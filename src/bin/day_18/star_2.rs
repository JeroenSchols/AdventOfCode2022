use std::collections::HashSet;
use itertools::Itertools;

pub fn run(pixels: &HashSet<(i8, i8, i8)>) -> usize {
    let adj: Vec<(i8, i8, i8)> = vec![(-1, 0, 0), (0, -1, 0), (0, 0, -1), (0, 0, 1), (0, 1, 0), (1, 0, 0)];
    let min_x = pixels.iter().map(|(x, _y, _z)| x).min().unwrap();
    let min_y = pixels.iter().map(|(_x, y, _z)| y).min().unwrap();
    let min_z = pixels.iter().map(|(_x, _y, z)| z).min().unwrap();
    let boundary = pixels.iter().map(|(x, y, z)| (x - min_x + 1, y - min_y + 1, z - min_z + 1)).collect::<HashSet<(i8, i8, i8)>>();
    let max_x = boundary.iter().map(|(x, _y, _z)| x).max().unwrap();
    let max_y = boundary.iter().map(|(_x, y, _z)| y).max().unwrap();
    let max_z = boundary.iter().map(|(_x, _y, z)| z).max().unwrap();
    let mut remaining = HashSet::new();
    for ((x, y), z) in (0..max_x + 2).cartesian_product(0..max_y + 2).cartesian_product(0..max_z + 2) {
        remaining.insert((x, y, z));
    }
    let mut stack = vec![(0, 0, 0)];
    let mut boundary_count = 0;
    remaining.remove(&(0, 0, 0));
    while !stack.is_empty() {
        let (x, y, z) = stack.pop().unwrap();
        for (dx, dy, dz) in &adj {
            if boundary.contains(&(x + dx, y + dy, z + dz)) {
                boundary_count += 1;
            } else if remaining.contains(&(x + dx, y + dy, z + dz)) {
                remaining.remove(&(x + dx, y + dy, z + dz));
                stack.push((x + dx, y + dy, z + dz));
            }
        }
    }
    boundary_count
}
