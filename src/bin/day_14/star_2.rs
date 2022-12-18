pub fn run(mut occupied: Vec<Vec<bool>>) -> u16 {
    let mut sand = 0;
    while !occupied[500][0] {
        let mut x = 500;
        let mut y = 0;
        loop {
            if y + 1 == occupied[0].len() {
                occupied[x][y] = true;
                break;
            } else if !occupied[x][y + 1] {
                y += 1;
            } else if !occupied[(x as isize - 1) as usize][y + 1] {
                x -= 1;
                y += 1;
            } else if !occupied[x + 1][y + 1] {
                x += 1;
                y += 1;
            } else {
                occupied[x][y] = true;
                break;
            }
        }
        sand += 1
    }
    sand
}
