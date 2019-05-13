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
        n: usize
    }
    let k = (n as f64).sqrt() as usize;

    // let mut i = n -1;
    // let mut sum = 0;
    // while k < i {
    //     let sh = n / i;
    //     let mo = n % i;
    //     if sh == mo {
    //         // println!("  {}", i);
    //         sum += i;
    //         i = n / (sh + 1);
    //         // i -= sh + 1;
    //     } else {
    //         i -= 1;
    //     }
    // }
    // println!("{}", sum);

    let mut m = mods(&n);
    m.sort();
    println!("{}", m.into_iter().rev().take_while(|v| v > &k).filter_map(|v| {
        let nv = v - 1;
        if n / nv == n % nv {
            Some(nv)
        } else {
            None
        }
    }).fold(0, |sum, a| sum + a));
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

