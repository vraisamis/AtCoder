// 日本語
use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let vs: Vec<u16> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
    print!("{}", vs[0] * vs[1] / 2);
}
