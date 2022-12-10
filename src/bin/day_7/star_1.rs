use std::collections::HashMap;

pub fn run(dir_sizes: &HashMap<String, u64>) -> u64 {
    dir_sizes.values().filter(|size| **size <= 100000).sum()
}
