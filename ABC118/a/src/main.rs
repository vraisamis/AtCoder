// 日本語
use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let vs: Vec<i64> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
    let a = vs[0];
    let b = vs[1];
    if b % a == 0 {
        println!("{}", a + b);
    } else {
        println!("{}", b - a);
    }
}
