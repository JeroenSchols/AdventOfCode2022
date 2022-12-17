use std::cmp::Ordering;
use crate::{cmp, Node};

pub(crate) fn run(input: &[(Node, Node)]) -> usize {
    input.iter()
        .enumerate()
        .filter_map(|(idx, (left, right))|
            if cmp(&left, &right) != Ordering::Greater {
                Some(idx + 1)
            } else {
                None
            }
        )
        .sum()
}
