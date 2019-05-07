// 日本語
use std::io::stdin;
use std::cmp;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let vs: Vec<u64> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
    let a = vs[0];
    let b = vs[1];
    let c = vs[2];
    let result = cmp::min(b / a, c);
    println!("{}", result);
}
