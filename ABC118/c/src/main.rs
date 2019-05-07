// 日本語
use std::io::stdin;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let vs: Vec<u64> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
    let n = vs[0];
    s = String::new();
    stdin().read_line(&mut s).ok();
    let mut vs: Vec<u64> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
    let vsli: Vec<u64> = vs.drain(1..).collect();
    
    let result = vsli.iter().fold(vs[0], |l, &r| { gcd(l, r) });
    println!("{}", result);
}
