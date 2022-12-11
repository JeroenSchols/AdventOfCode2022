use crate::{Monkey, Operation};

pub fn run(mut monkeys: Vec<Monkey>) -> u64 {
    let mut mod_val = 1u64;
    for monkey in &monkeys {
        mod_val *= monkey.div_val;
    }
    let mut items_handled = vec![0u64; monkeys.len()];
    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            let mut new_items: Vec<(usize, u64)> = Vec::new();
            for item in &monkeys[m].items {
                items_handled[m] += 1;
                let new_item: u64 = match monkeys[m].op {
                    Operation::Plus => {(item + monkeys[m].op_val) % mod_val},
                    Operation::Mult => {(item * monkeys[m].op_val) % mod_val},
                    Operation::Square => {(item * item) % mod_val}
                };
                if new_item % monkeys[m].div_val == 0 {
                    new_items.push((monkeys[m].true_monkey, new_item));
                } else {
                    new_items.push((monkeys[m].false_monkey, new_item));
                }
            }
            monkeys[m].items.clear();
            for (n, item) in new_items {
                monkeys[n].items.push(item);
            }
        }
    }
    items_handled.sort();
    items_handled[items_handled.len()-2] * items_handled[items_handled.len()-1]
}
