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
        aa: [usize; n],
    }
    let cumsum = {
        let mut cumsum = vec![0; n + 1];
        for i in 1..n + 1 {
            cumsum[i] = cumsum[i - 1] + aa[i - 1];
        }
        cumsum
    };

    let mut cnt = 0;
    for start in 0..n + 1 {
        if cumsum[n] - cumsum[start] < k {
            break;
        }
        let f = |i| cumsum[i] - cumsum[start];
        let x = lower_bound(f, &k, start + 1, n + 1);
        cnt += n + 1 - x;
        // for end in start + 1 .. n + 1 {
        //     let v = cumsum[end] - cumsum[start];
        //     if v >= k {
        //         cnt += n + 1 - end;
        //         break;
        //     }
        // }
    }
    println!("{}", cnt);
}


#[allow(dead_code)]
fn lower_bound<F, T>(f: F, x: &T, left: usize, right: usize) -> usize
where T: Ord, F: Fn(usize) -> T {
    let mut left = left;
    let mut right = right;
    while left < right {
        let mid = (left + right) / 2;
        let value = f(mid);
        if x <= &value {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    return right;
}

#[allow(dead_code)]
fn upper_bound<F, T>(f: F, x: &T, len: usize) -> usize
where T: Ord, F: Fn(usize) -> T {
    let mut left = 0;
    let mut right = len;
    while left < right {
        let mid = (left + right) / 2;
        let value = f(mid);
        if x < &value {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    return left;
}
