// 日本語
use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let vs: Vec<u32> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
    let mut a = vec![0, vs[0]];
    let mut lasta = vs[0];
    for i in 2..1_000_000 {
        let an = if lasta % 2 == 0 {
            lasta / 2
        } else {
            3 * lasta + 1
        };
        if a.iter().any(|ak| *ak == an) {
            print!("{}", i);
            return;
        } else {
            a.push(an);
            lasta = an;
        }
    }
}
