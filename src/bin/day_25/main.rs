fn dec_to_snafu(mut dec: i64) -> String {
    let mut rev_snafu = String::new();
    let mut carry = 0;
    while dec + carry > 0 {
        let mut digit = dec % 5 + carry;
        dec /= 5;
        carry = 0;
        if digit >= 3 {
            digit -= 5;
            carry = 1;
        }
        rev_snafu += match digit {
            -2 => "=",
            -1 => "-",
            0 => "0",
            1 => "1",
            2 => "2",
            _ => unreachable!()
        };
    }
    rev_snafu.chars().rev().collect()
}

fn snafu_to_dec(snafu: &str) -> i64 {
    let mut dec = 0;
    for c in snafu.chars() {
        dec = dec * 5 + match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!()
        };
    }
    dec
}

fn main() {
    let file = "src/bin/day_25/input.txt";
    let out = dec_to_snafu(std::fs::read_to_string(file).unwrap().lines()
        .map(snafu_to_dec).sum::<i64>());
    println!("star: {out}");
}
