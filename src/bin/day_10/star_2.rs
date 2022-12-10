fn tick(time: &mut i32, strength: &i32, result: &mut String) {
    let pos = *time % 40;
    *time += 1;
    if pos.abs_diff(*strength) <= 1 {
        result.push('#');
    } else {
        result.push(' ');
    }
    if pos == 39 {
        result.push('\n');
    }
}

pub fn run(input_file: &str) -> String {
    let mut result = String::new();
    let mut time = 0;
    let mut strength = 1;
    for line in std::fs::read_to_string(input_file)
        .unwrap()
        .lines()
    {
        if line == "noop" {
            tick(&mut time, &strength, &mut result);
        } else {
            tick(&mut time, &strength, &mut result);
            tick(&mut time, &strength, &mut result);
            strength += line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();
        }
    }
    result
}
