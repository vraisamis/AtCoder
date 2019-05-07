use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::cmp::min;

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

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

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
        aa: [usize; n]
    }

    // let mut aa = aa;
    // aa.sort();

    let mut vv = mods(&aa[0]);
    vv.append(&mut mods(&aa[1]));
    let result = vv.iter().filter(|v| {
        let cnt = aa.iter().filter(|a| *a % **v == 0).count();
        cnt >= n - 1
    }).max().unwrap();
    // let result = (0..n).map(|i| {
    //     let mut c = if i != 0 { aa[0] } else {aa[1] };
    //     for j in 1..n {
    //         if i != j {
    //             c = gcd(c, aa[j]);
    //         }
    //     }
    //     c
    // }).max().unwrap();
    // let mut m: BTreeMap<usize, usize> = BTreeMap::new();
    //
    // let mut debugv = vec![];
    // for a in aa {
    //     let vv = mods(&a);
    //     debugv.push(vv);
    //     // for v in vv {
    //     //     let k = *m.get(&v).unwrap_or(&0);
    //     //     m.insert(v, k + 1);
    //     // }
    // }
    // println!("DEBUG:{}", debugv.len());
    // // let mut index = 0;
    // // let mut count = 0;
    // // for (k, v) in m {
    // //     let v = min(v, n - 1);
    // //     match count.cmp(&v) {
    // //         Ordering::Less => {
    // //             // update
    // //             index = k;
    // //             count = v;
    // //         },
    // //         Ordering::Equal => {
    // //             if index < k {
    // //                 index = k;
    // //             }
    // //         }
    // //         Ordering::Greater => ()
    // //     }
    // // }
    // // println!("{}", index);
    println!("{}", result);
}

fn mods(n: &usize) -> Vec<usize> {
    match n {
        &0 => vec![],
        &1 => vec![1],
        _ => {
            let mut v = vec![];
            let mut i = 1;
            while i * i <= *n {
                if n % i == 0 {
                    v.push(i);
                    v.push(n / i);
                }
                i += 1;
            }
            v
        }
    }
}

fn gcd(l: usize, r: usize) -> usize {
    if r == 0 {
        l
    } else {
        gcd(r, l % r)
    }
}
