// 日本語

use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let vs: Vec<u32> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
    let n = vs[0];
    let k = vs[1];
    if (n + 1) / 2 >= k {
        println!("YES");
    } else {
        println!("NO");
    }
}
