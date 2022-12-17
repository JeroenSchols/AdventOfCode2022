mod star_1;
mod star_2;

use std::cmp::Ordering;

#[derive(Default, Clone)]
struct Node {
    children: Vec<Node>,
    val: Option<u8>,
}

fn cmp(left: &Node, right: &Node) -> Ordering {
    match (left.val, right.val) {
        (Some(l), Some(r)) => l.cmp(&r),
        (Some(_), None) => {
            let mut node = Node::default();
            node.children.push(left.clone());
            cmp(&node, right)
        }
        (None, Some(_)) => {
            let mut node = Node::default();
            node.children.push(right.clone());
            cmp(left, &node)
        }
        (None, None) => {
            for i in 0.. {
                let ordering = match (left.children.get(i), right.children.get(i)) {
                    (Some(lchild), Some(rchild)) => cmp(lchild, rchild),
                    (Some(_), None) => return Ordering::Greater,
                    (None, Some(_)) => return Ordering::Less,
                    (None, None) => return Ordering::Equal
                };
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
            unreachable!();
        }
    }
}

fn parse_line(line: &str, idx: usize) -> Option<(Node, usize)> {
    if idx == line.len() {
        return None;
    }
    let mut root = Node::default();
    if *line.as_bytes().get(idx).unwrap() == b'[' {
        if *line.as_bytes().get(idx + 1).unwrap() == b']' {
            return Some((root, idx + 2));
        }
        let mut eidx = idx;
        while *line.as_bytes().get(eidx).unwrap() != b']' {
            let (child, neidx) = parse_line(&line, eidx + 1).unwrap();
            eidx = neidx;
            root.children.push(child);
        }
        Some((root, eidx + 1))
    } else {
        let eidx = match line[idx..].find(',') {
            None => line.len(),
            Some(l) => idx + l
        }.min(match line[idx..].find(']') {
            None => line.len(),
            Some(l) => idx + l
        });
        root.val = Some(line[idx..eidx].parse::<u8>().unwrap());
        Some((root, eidx))
    }
}

fn parse_file(input_file: &str) -> Vec<(Node, Node)> {
    std::fs::read_to_string(input_file).unwrap()
        .split("\r\n\r\n")
        .map(|entry| {
            let mut line = entry.lines();
            let (left, _) = parse_line(line.next().unwrap(), 0).unwrap();
            let (right, _) = parse_line(line.next().unwrap(), 0).unwrap();
            (left, right)
        })
        .collect()
}

fn main() {
    let input_file = "src/bin/day_13/input.txt";
    let input = parse_file(input_file);
    let out_1 = star_1::run(&input);
    println!("star 1: {out_1:?}");
    let out_2 = star_2::run(&input);
    println!("star 2: {out_2:?}");
}
