use std::collections::HashMap;

pub fn run(dir_sizes: &HashMap<String, u64>) -> u64 {
    let target_size = dir_sizes[""] - 40000000;
    *dir_sizes.values()
        .filter(|size| **size >= target_size)
        .min()
        .unwrap()
}
