// 日本語
use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let vs: Vec<u64> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
    let n = vs[0];
    let m = vs[1] as usize;

    let mut x: Vec<bool> = vec![true; m + 1];

    for _ in 0..n {
        s = String::new();
        stdin().read_line(&mut s).ok();
        let mut vs: Vec<u64> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
        let vs: Vec<u64> = vs.drain(1..).collect();
        x = x.iter().enumerate().map(|(i, xi)| { *xi && vs.contains(&(i as u64)) }).collect();
    }
    let result = x.iter().enumerate().map(|(i, xi)| { if *xi { i as u64 } else { 0 } }).filter(|i| { *i > 0 }).count();
    println!("{}", result);
}
