use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn run(input_file: &str) -> u64 {
    let input = std::fs::read_to_string(input_file).unwrap();
    let calorie_sums = input.split("\r\n\r\n")
        .map(|elf| elf.lines().map(|str| str.parse::<u64>().unwrap()).sum::<u64>())
        .map(Reverse);

    let mut min_heap = BinaryHeap::with_capacity(4);
    for calorie_sum in calorie_sums {
        min_heap.push(calorie_sum);
        if min_heap.len() == 4 {
            min_heap.pop();
        }
    }

    min_heap.into_iter().map(|Reverse(cal)| cal).sum()
}
