#[allow(unused_imports)]
use std::cmp::{min, max};
use std::collections::VecDeque;

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
        k: usize,
        vv: [i64; n],
    }
    let mut vv: VecDeque<i64> = vv.into_iter().collect();
    let mut have = vec![];

    let result = solve(&mut vv, &mut have, k, 0);
    println!("{}", result);
}

fn solve(vv: &mut VecDeque<i64>, have: &mut Vec<i64>, k: usize, i: usize) -> i64 {
    if i >= k {
        return 0;
    }
    let mut result = 0;
    // pull
    if vv.len() > 0 {
        let left = vv.front().unwrap().clone();
        let right = vv.back().unwrap().clone();
        match left.cmp(&right) {
            std::cmp::Ordering::Equal => {
                // left
                let v = vv.pop_front().unwrap();
                have.push(v);
                result = max(result, v + solve(vv, have, k, i + 1));
                have.pop();
                vv.push_front(v);
                // right
                let v = vv.pop_back().unwrap();
                have.push(v);
                result = max(result, v + solve(vv, have, k, i + 1));
                have.pop();
                vv.push_back(v);
            },
            std::cmp::Ordering::Greater => {
                // left
                let v = vv.pop_front().unwrap();
                have.push(v);
                result = max(result, v + solve(vv, have, k, i + 1));
                have.pop();
                vv.push_front(v);
            },
            std::cmp::Ordering::Less => {
                // right
                let v = vv.pop_back().unwrap();
                have.push(v);
                result = max(result, v + solve(vv, have, k, i + 1));
                have.pop();
                vv.push_back(v);
            },
        }
    }
    if have.len() > 0 {
        let (h, &value) = have.iter().enumerate().min_by(|&(_, lv), &(_, rv)| {
            lv.cmp(&rv)
        }).unwrap();
        let mut new_have = have.iter().enumerate().filter_map(|(hi, hv)| {
            if hi == h {
                None
            } else {
                Some(*hv)
            }
        }).collect();
        // left
        vv.push_front(value);
        result = max(result, solve(vv, &mut new_have, k, i + 1) - value);
        vv.pop_front();
        // right
        vv.push_back(value);
        result = max(result, solve(vv, &mut new_have, k, i + 1) - value);
        vv.pop_back();
    }
    result
}
