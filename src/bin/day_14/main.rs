mod star_1;
mod star_2;

use itertools::max;

fn create_map(input_file: &str) -> Vec<Vec<bool>> {
    let input: Vec<Vec<(usize, usize)>> = std::fs::read_to_string(input_file).unwrap()
        .lines().map(|line| line.split(" -> ")
        .map(|coords| {
            let mut coord = coords.split(',');
            (coord.next().unwrap().parse::<usize>().unwrap(), coord.next().unwrap().parse::<usize>().unwrap())
        }).collect()
    ).collect();
    let max_x = max(input.iter().flat_map(|row| row.iter().map(|(x, _y)| x))).unwrap();
    let max_y = max(input.iter().flat_map(|row| row.iter().map(|(_x, y)| y))).unwrap();
    let mut occupied = vec![vec![false; max_y + 2]; max_x + max_y];
    for row in input {
        for i in 1..row.len() {
            let min_x = row[i - 1].0.min(row[i].0);
            let max_x = row[i - 1].0.max(row[i].0);
            let min_y = row[i - 1].1.min(row[i].1);
            let max_y = row[i - 1].1.max(row[i].1);
            for x in min_x..max_x + 1 {
                for y in min_y..max_y + 1 {
                    occupied[x][y] = true;
                }
            }
        }
    }
    occupied
}

fn main() {
    let file = "src/bin/day_14/input.txt";
    let map = create_map(file);
    let out_1 = star_1::run(map.clone());
    println!("star 1: {out_1:?}");
    let out_2 = star_2::run(map.clone());
    println!("star 2: {out_2:?}");
}
