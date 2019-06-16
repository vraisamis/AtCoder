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
        mut edges: [(usize1, usize1); n - 1],
        mut c: [usize; n],
    }
    let oe = edges.clone();
    let mut e2 = edges.iter().map(|&(l, r)| (r, l)).collect();
    edges.append(&mut e2);
    // 連結してる順番でやらなくていいのか？
    edges.sort_by(|&(ll, lr), &(rl, rr)| {
        match ll.cmp(&rl) {
            std::cmp::Ordering::Equal => lr.cmp(&rr),
            other => other
        }
    });
    c.sort();
    let mut run = vec![(0, None)];
    // (Some(value), Some(parent)
    let mut results = vec![(None, None); n];
    while let Some((l, p)) = run.pop() {
        let (tv, _) = results[l];
        if tv.is_some() {
            continue;
        }
        // ireru
        results[l] = (c.pop(), p);
        sort(&mut results, p);
        for &(_, right) in edges.iter().filter(|&&(lv, _)| lv == l) {
            run.push((right, Some(l)));
        }
    }

    // sum
    let mut sum = 0;
    for (l, r) in oe {
        let lv = results[l].0.unwrap();
        let rv = results[r].0.unwrap();
        sum += min(lv, rv);
    }
    println!("{}", sum);
    let mut prefix = "";
    for (v, _) in results {
        let v = v.unwrap();
        print!("{}{}", prefix, v);
        prefix = " ";
    }
    println!("");
}

fn sort(v: &mut Vec<(Option<usize>, Option<usize>)>, k: Option<usize>) {
    if k.is_none() {
        return;
    }
    let k = k.unwrap();
    let (value, parent) = v[k];
    if parent.is_none() {
        return;
    }
    let parent = parent.unwrap();
    let (pvalue, pparent) = v[parent];
    if value.is_none() || pvalue.is_none() {
        return;
    }
    let (value, pvalue) = (value.unwrap(), pvalue.unwrap());
    if value > pvalue {
        v[k] = (Some(pvalue), Some(parent));
        v[parent] = (Some(value), pparent);
        sort(v, Some(parent));
    }
}
