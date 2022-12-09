mod star_1;
mod star_2;

fn main() {
    let file = "src/bin/day_1/input.txt";
    let out_1 = star_1::run(file);
    println!("star 1: {out_1:?}");
    let out_2 = star_2::run(file);
    println!("star 2: {out_2:?}");
}
