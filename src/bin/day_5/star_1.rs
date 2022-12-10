pub fn run(stacks: &[Vec<char>], moves: &[(usize, usize, usize)]) -> String {
    let mut stacks_copy = stacks.to_vec();
    for (count, from_idx, to_idx) in moves {
        for _ in 0..*count {
            if let Some(c) = stacks_copy[*from_idx].pop() {
                stacks_copy[*to_idx].push(c)
            }
        }
    }
    stacks_copy.iter().filter_map(|stack| stack.last()).collect()
}
