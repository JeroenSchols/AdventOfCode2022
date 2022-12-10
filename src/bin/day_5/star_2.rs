pub fn run(stacks: &[Vec<char>], moves: &[(usize, usize, usize)]) -> String {
    let mut stacks_copy = stacks.to_vec();
    for (count, from_idx, to_idx) in moves {
        let from_stack_len = stacks_copy[*from_idx].len();
        let moving_crates = stacks_copy[*from_idx].split_off(from_stack_len - *count);
        stacks_copy[*to_idx].extend(moving_crates);
    }
    stacks_copy.iter().filter_map(|stack| stack.last()).collect()
}
