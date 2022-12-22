use itertools::Itertools;

pub fn run(input_file: &str) -> i32 {
    let numbers = std::fs::read_to_string(input_file).unwrap()
        .lines().map(|line| line.parse::<i32>().unwrap()).collect_vec();
    let mut indices = (0..numbers.len()).collect_vec();
    for (num_idx, num_delta) in numbers.iter().enumerate() {
        let indices_idx = indices.iter().position(|idx| *idx == num_idx).unwrap();
        let mut abs_delta = *num_delta % (numbers.len() as i32 - 1);
        if abs_delta < 0 {
            abs_delta += numbers.len() as i32 - 1;
        }
        for delta in 0..abs_delta as usize {
            indices[(indices_idx + delta) % numbers.len()] = indices[(indices_idx + delta + 1) % numbers.len()]
        }
        indices[(indices_idx + abs_delta as usize) % numbers.len()] = num_idx;
    }
    let zero_indices_idx = indices.iter().position(|idx| numbers[*idx] == 0).unwrap();
    numbers[indices[(zero_indices_idx + 1000) % numbers.len()]] + numbers[indices[(zero_indices_idx + 2000) % numbers.len()]] + numbers[indices[(zero_indices_idx + 3000) % numbers.len()]]
}
