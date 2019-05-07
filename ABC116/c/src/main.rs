// 日本語
use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let n: usize = s.trim().parse().ok().unwrap();
    s = String::new();
    stdin().read_line(&mut s).ok();
    let hs: Vec<u32> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();

    let mut sum = 0;
    let max = *(hs.iter().max().unwrap());
    for m in (1..max + 1).rev() {
        let mut tmp = 0;
        for i in 0..n {
            if hs[i] >= m && (i == 0 || hs[i - 1] < m) {
                    tmp = tmp + 1;
            }
        }
        sum = sum + tmp;
    }
    print!("{}", sum);
}
