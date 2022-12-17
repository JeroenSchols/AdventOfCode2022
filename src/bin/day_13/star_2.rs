use std::cmp::Ordering;
use crate::{cmp, Node};

fn create_divider_node(val: u8) -> Node {
    Node {
        children: vec![Node {
            children: vec![Node {
                children: vec![],
                val: Some(val),
            }],
            val: None,
        }],
        val: None,
    }
}

pub(crate) fn run(input: &[(Node, Node)]) -> usize {
    let mut flat_input = Vec::new();
    for (left, right) in input {
        flat_input.push(left);
        flat_input.push(right);
    }
    let div_1 = create_divider_node(2);
    let div_2 = create_divider_node(6);
    let idx_1 = flat_input.iter().filter(|node| cmp(node, &div_1) == Ordering::Less).count();
    let idx_2 = flat_input.iter().filter(|node| cmp(node, &div_2) == Ordering::Less).count();
    (idx_1 + 1) * (idx_2 + 2)
}
