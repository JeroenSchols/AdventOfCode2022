mod star_1;
mod star_2;

#[derive(Clone)]
pub enum Operation {
    Mult, Plus, Square
}

#[derive(Clone)]
pub struct Monkey {
    items: Vec<u64>,
    op: Operation,
    op_val: u64,
    div_val: u64,
    true_monkey: usize,
    false_monkey: usize
}

fn parse_monkeys(input_file: &str) -> Vec<Monkey> {
    std::fs::read_to_string(input_file).unwrap()
        .split("\r\n\r\n")
        .map(|block| {
            let mut iter = block.lines();
            iter.next();
            let items: Vec<u64> = iter.next().unwrap()
                .strip_prefix("  Starting items: ").unwrap()
                .split(", ")
                .map(|item| item.parse::<u64>().unwrap()).collect();
            let op_val_str = iter.next().unwrap()
                .strip_prefix("  Operation: new = old ").unwrap();
            let op;
            let op_val;
            if op_val_str == "* old" {
                op = Operation::Square;
                op_val = 0;
            } else if op_val_str.starts_with('*') {
                op = Operation::Mult;
                op_val = op_val_str.strip_prefix("* ").unwrap().parse::<u64>().unwrap();
            } else {
                op = Operation::Plus;
                op_val = op_val_str.strip_prefix("+ ").unwrap().parse::<u64>().unwrap();
            }
            let div_val = iter.next().unwrap()
                .strip_prefix("  Test: divisible by ").unwrap()
                .parse::<u64>().unwrap();
            let true_monkey = iter.next().unwrap()
                .strip_prefix("    If true: throw to monkey ").unwrap()
                .parse::<usize>().unwrap();
            let false_monkey = iter.next().unwrap()
                .strip_prefix("    If false: throw to monkey ").unwrap()
                .parse::<usize>().unwrap();
            Monkey {items, op, op_val, div_val, true_monkey, false_monkey}
        })
        .collect()
}

fn main() {
    let monkeys = parse_monkeys("src/bin/day_11/input.txt");
    let out_1 = star_1::run(monkeys.to_vec());
    println!("star 1: {out_1:?}");
    let out_2 = star_2::run(monkeys.to_vec());
    println!("star 2: {out_2:?}");
}
