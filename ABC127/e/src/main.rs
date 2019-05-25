#[allow(unused_imports)]
use std::cmp::{min, max};

use std::collections::BTreeSet;

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

const DIV: usize = 1000000000 + 7;

fn main() {
    input!{
        n: usize,
        m: usize,
        k: usize,
    }
    let mut set: BTreeSet<usize> = BTreeSet::new();
    set.insert(0);
    let result = solve(&mut set, n, m, 1, k, 1, 0);
    println!("{}", result);
}

fn solve(set: &mut BTreeSet<usize>, n: usize, m: usize, nmi: usize, k: usize, ki: usize, minisum: usize) -> usize {
    if ki >= k {
        return minisum;
    }
    let mut sum = 0;
    for row in nmi..n * m {
        let mut minisum = minisum + delta_cost(set, n, row);
        minisum %= DIV;
        set.insert(row);
        sum += solve(set, n, m, row + 1, k, ki + 1, minisum);
        set.remove(&row);
        sum %= DIV;
    }
    sum
}

fn delta_cost(set: &BTreeSet<usize>, n: usize, i: usize) -> usize {
    let mut sum = 0;
    for s in set {
        let lx = s / n;
        let ly = s % n;
        let rx = i / n;
        let ry = i % n;
        sum += max(lx, rx) - min(lx, rx) + max(ly, ry) - min(ly, ry);
        sum %= DIV;
    }
    // sum
}
