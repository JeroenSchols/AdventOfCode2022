use std::collections::HashMap;

mod star_1;
mod star_2;

fn main() {
    let file = "src/bin/day_7/input.txt";
    let mut pwd: Vec<String> = vec!["".to_string()];
    let mut dir_sizes: HashMap<String, u64> = HashMap::new();
    for line in std::fs::read_to_string(file).unwrap().lines() {
        if line == "$ ls" || line == "$ cd /" || line.starts_with("dir") {
            continue;
        }
        match line.strip_prefix("$ cd ") {
            Some("..") => {
                pwd.pop();
                continue;
            }
            Some(dir) => {
                pwd.push(dir.parse().unwrap());
                continue;
            }
            None => {}
        }
        let size = line.split_whitespace().next().unwrap().parse::<u64>().unwrap();
        let mut path = String::new();
        for dir in &pwd {
            path.push_str(dir.as_str());
            *dir_sizes.entry(path.to_string()).or_insert(0) += size;
            path.push('/');
        }
    }

    let out_1 = star_1::run(&dir_sizes);
    println!("star 1: {out_1:?}");
    let out_2 = star_2::run(&dir_sizes);
    println!("star 2: {out_2:?}");
}
