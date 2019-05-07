// 日本語
use std::io::{stdin, Read};

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let nk: Vec<u32> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
    let n = nk[0];
    let k = nk[1];
    let mut t: Vec<u32> = vec![];
    let mut d: Vec<u32> = vec![];
    s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    for line in s.lines() {
        let tmp: Vec<u32> = line.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
        t.push(tmp[0]);
        d.push(tmp[1]);
    }
    // for i in n {
    // }
    //
    // let mut sum = 0;
    // let max = *(hs.iter().max().unwrap());
    // for m in (1..max + 1).rev() {
    //     let mut tmp = 0;
    //     for i in 0..n {
    //         if hs[i] >= m && (i == 0 || hs[i - 1] < m) {
    //                 tmp = tmp + 1;
    //         }
    //     }
    //     sum = sum + tmp;
    // }
    // print!("{}", sum);
}
