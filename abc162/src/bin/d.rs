use std::cmp::{max, min};
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

fn main() {
    input! {
        _n: usize,
        s: chars
    }
    let rs: BTreeSet<usize> = s
        .iter()
        .enumerate()
        .filter_map(|(index, &c)| if c == 'R' { Some(index) } else { None })
        .collect();
    let gs: BTreeSet<usize> = s
        .iter()
        .enumerate()
        .filter_map(|(index, &c)| if c == 'G' { Some(index) } else { None })
        .collect();
    let bs: BTreeSet<usize> = s
        .iter()
        .enumerate()
        .filter_map(|(index, &c)| if c == 'B' { Some(index) } else { None })
        .collect();

    // println!("rs: {:?}", rs);
    // println!("gs: {:?}", gs);
    // println!("bs: {:?}", bs);

    let mut cnt = 0;
    for &ri in rs.iter() {
        for &gi in gs.iter() {
            let d = max(ri, gi) - min(ri, gi);
            let mut search = vec![max(ri, gi) + d];
            if d % 2 == 0 {
                search.push(max(ri, gi) - (d / 2));
            }
            if min(ri, gi) > d {
                search.push(min(ri, gi) - d);
            }
            let i = search.iter().filter(|s| bs.contains(s)).count();
            cnt += bs.len() - i;
            // for &bi in bs.iter() {
            //     let mut v = vec![ri, gi, bi];
            //     v.sort();
            //     let d1 = v[1] - v[0];
            //     let d2 = v[2] - v[1];
            //     // このケース、ri, giに対して高々1つじゃない？？？
            //     if d1 != d2 {
            //         cnt += 1;
            //     }
            // }
        }
    }

    println!("{}", cnt);
}
