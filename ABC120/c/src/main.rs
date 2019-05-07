// 日本語
use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let vs: Vec<i16> = s.trim().chars().map(|token| token.to_string().parse().ok().unwrap()).collect();

    let mut i = 0;
    let mut result = 0;
    for v in vs {
        // cond
        if (i > 0 && v == 0) || (i < 0 && v == 1) {
            result += 1;
        }
        // stack
        i += if v == 1 { 1 } else { -1 };
    }
    println!("{}", result * 2);
}
