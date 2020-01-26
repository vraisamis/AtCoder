#[allow(unused_imports)]
use std::cmp::{min, max};

#[allow(dead_code)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(dead_code)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(dead_code)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input! {
        h: i64,
        n: usize,
        magic: [(i64, i64); n],
    }

    // DP
    // とりあえずでかい数
    // iダメージを与えるのに必要なMP dp[i]
    // ダメージ0でいいならMP0
    let mut dp = vec![0];
    // dp[0] = 0;
    for v in 1..(h + 1) {
        let mut min_mp = 1_000_000_000;
        // i番目の魔法を使うときの最小MPをresultにもとめて、それがvダメージ与える最小MPならmin_mpを更新する
        for i in 0..n {
            let current_magic = magic[i];
            let (cma, cmb) = (current_magic.0, current_magic.1);
            let others = if v > cma {
                dp[(v - cma) as usize]
            } else {
                0
            };
            min_mp = min(min_mp, cmb + others);
        }
        dp.push(min_mp);
    }

    println!("{}", dp.pop().unwrap());
}
