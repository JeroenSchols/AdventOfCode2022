fn tick(time: &mut i32, strength: &i32, result: &mut i32) {
    *time += 1;
    if *time % 40 == 20 {
        *result += *time * strength;
    }
}

pub fn run(input_file: &str) -> i32 {
    let mut result = 0;
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
