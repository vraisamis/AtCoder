// 日本語
use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let vs: Vec<u64> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
    let a = vs[0];
    let b = vs[1];
    let k = vs[2];

    let mut n = 0;
    for i in (1..gcd(a, b)+1).rev() {
        if a % i == 0 && b % i == 0 {
            n += 1;
            if n == k {
                println!("{}", i);
                break;
            }
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
