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
        mut aa: [u8; n]
    }
    let mut bb: Vec<Option<u8>> = vec![None; n];
    let mut sum: u64 = 0;
    for k in (0..n).rev() {
        let (ia, va) = (k, aa[k]);
        bb[ia] = Some(va);
        sum += va as u64;
        for i in primes(ia + 1) {
            // println!("deb: {} ({})", i, k);
            aa[i - 1] ^= va;
        }
    }
    // let f = |i, bb: &Vec<Option<u8>>| {
    //     bb
    //         .iter()
    //         .enumerate()
    //         .filter_map(|(bi, &bv)| if (bi + 1) % (i + 1) == 0 { bv } else { None })
    //         .fold(0, |sum, b| sum ^ b)
    // };
    // let mut sum: u64 = 0;
    // for (ia, &va) in aa.iter().enumerate().rev() {
    //     let r = f(ia, &bb);
    //     let v = r ^ va;
    //     bb[ia] = Some(v);
    //     sum += v as u64;
    // }





    println!("{}", sum);
    if sum > 0 {
        let bii: Vec<_> = bb.iter().enumerate().filter_map(|(bi, &bv)| {
            let v = match bv {
                Some(v) => v,
                None => return None
            };
            if v == 1 {
                // 0 -> 1, n - 1 -> n
                Some(bi + 1)
            } else {
                None
            }
        }).collect();
        print!("{}", bii[0]);
        for i in bii.iter().skip(1) {
            print!(" {}", i);
        }
        println!("");
    }
}




fn primes(v: usize) -> Vec<usize> {
    let mut ret = vec![1, v];
    let mut i = 2;
    while i * i <= v {
        if v % i == 0 {
            ret.push(i);
            if i * i == v {
                return ret;
            }
            ret.push(v / i);
        }
        i += 1;
    }
    ret
}
