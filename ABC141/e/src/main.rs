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
    input!{
        n: usize,
        s: String
    }
    let mut m = 0;
    for i in 1..n {
        let s1 = s.chars().take(i).collect::<Vec<char>>();
        let s2 = s.chars().skip(i).collect::<Vec<char>>();
        m = max(m, LCS(&s1, &s2));
        // println!("test: s1[{:?}], s2[{:?}], m[{}]", s1, s2, m);
    }
    println!("{}", m);
}

fn LCS(s1: &Vec<char>, s2: &Vec<char>) -> usize {
    let l1 = s1.len();
    let l2 = s2.len();
    let mut m = vec![vec![0; l2]; l1];
    {
        let mut v = 0;
        for i in 0..l1 {
            if i < l2 && s1[i] == s2[i] {
                v = 1;
            }
            m[i][0] = v;
        }
    }
    {
        let mut v = 0;
        for i in 0..l2 {
            if i < l1 && s1[i] == s2[i] {
                v = 1;
            }
            m[0][i] = v;
        }
    }
    // println!("!!!m: {:?}", m);
    let mut max_len = 0;
    for i in 0..l1-1 {
        for j in 0..l2-1 {
            if s1[i + 1] == s2[j + 1] {
                m[i + 1][j + 1] = m[i][j] + 1;
                max_len = max(m[i + 1][j + 1], max_len);
            } else {
                m[i + 1][j + 1] = max(m[i][j + 1], m[i + 1][j]);
            }
        }
        // println!("runrun!!!m: {:?}", m);
    }
    max_len
}
