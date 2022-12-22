use std::collections::HashMap;

enum Operation {
    Plus,
    Minus,
    Mult,
    Div,
}

struct Monkey {
    operation: Option<Operation>,
    left: String,
    right: String,
    val: Option<i64>,
}

fn parse_input(input_file: &str) -> HashMap<String, Monkey> {
    std::fs::read_to_string(input_file).unwrap().lines()
        .map(|line| {
            let mut split = line.split(": ");
            let name = split.next().unwrap();
            let val = split.next().unwrap();
            if let Ok(v) = val.parse::<i64>() {
                return (name.to_string(), Monkey {
                    operation: None,
                    left: "".to_string(),
                    right: "".to_string(),
                    val: Some(v),
                });
            }
            split = val.split(" ");
            let left = split.next().unwrap();
            let op = match split.next().unwrap() {
                "+" => Operation::Plus,
                "-" => Operation::Minus,
                "*" => Operation::Mult,
                "/" => Operation::Div,
                _ => unreachable!()
            };
            let right = split.next().unwrap();
            (name.to_string(), Monkey {
                operation: Some(op),
                left: left.to_string(),
                right: right.to_string(),
                val: None,
            })
        }).collect()
}

fn calculate(monkeys: &HashMap<String, Monkey>, name: &String) -> Option<i64> {
    if monkeys[name].operation.is_none() {
        monkeys[name].val
    } else if let (Some(left_val), Some(right_val)) = (calculate(monkeys, &monkeys[name].left), calculate(monkeys, &monkeys[name].right)) {
        Some(match monkeys[name].operation.as_ref().unwrap() {
            Operation::Plus => left_val + right_val,
            Operation::Minus => left_val - right_val,
            Operation::Mult => left_val * right_val,
            Operation::Div => left_val / right_val
        })
    } else {
        None
    }
}

fn derive(monkeys: &HashMap<String, Monkey>, name: &String, target: i64) -> i64 {
    if monkeys[name].operation.is_none() {
        return target;
    }
    match (calculate(monkeys, &monkeys[name].left), calculate(monkeys, &monkeys[name].right)) {
        (Some(left_val), None) => match monkeys[name].operation.as_ref().unwrap() {
            Operation::Plus => derive(monkeys, &monkeys[name].right, target - left_val),
            Operation::Minus => derive(monkeys, &monkeys[name].right, left_val - target),
            Operation::Mult => derive(monkeys, &monkeys[name].right, target / left_val),
            Operation::Div => derive(monkeys, &monkeys[name].right, left_val / target)
        },
        (None, Some(right_val)) => match monkeys[name].operation.as_ref().unwrap() {
            Operation::Plus => derive(monkeys, &monkeys[name].left, target - right_val),
            Operation::Minus => derive(monkeys, &monkeys[name].left, target + right_val),
            Operation::Mult => derive(monkeys, &monkeys[name].left, target / right_val),
            Operation::Div => derive(monkeys, &monkeys[name].left, target * right_val)
        },
        _ => unreachable!()
    }
}

fn main() {
    let file = "src/bin/day_21/input.txt";
    let mut monkeys = parse_input(file);

    let out_1 = calculate(&monkeys, &"root".to_string()).unwrap();
    println!("star 1: {out_1}");

    monkeys.get_mut("root").unwrap().operation = Some(Operation::Minus);
    monkeys.get_mut("humn").unwrap().val = None;
    let out_2 = derive(&monkeys, &"root".to_string(), 0);
    println!("star 2: {out_2}");
}
