// 日本語

use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let vs: Vec<u64> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
    let k = vs[0];
    let a = vs[1];
    let b = vs[2];

    if b <= a + 2 {
        println!("{}", k + 1);
        return;
    }

    let mut n:u64 = 1;
    let mut yen:u64 = 0;
    // シミュレーション方針
    for i in (0..k).rev() {
        // println!("{}", i);
        if i <= yen {
            if yen > 0 {
                n += b;
                yen -= 1;
                // println!("  yen to n n:{}, yen:{}", n, yen);
            } else {
                n += 1;
                // println!("  n++ n:{}, yen:{}", n, yen);
            }
        } else if n >= a {
            n -= a;
            yen += 1;
            // println!("  n to yen n:{}, yen:{}", n, yen);
        } else if yen > 0 {
            n += b;
            yen -= 1;
            // println!("  yen to n n:{}, yen:{}", n, yen);
        } else {
            n += 1;
            // println!("  n++ n:{}, yen:{}", n, yen);
        }
    }
    println!("{}", n);
}
