use std::collections::BTreeMap;
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
        n: usize
    }

    let mut m: BTreeMap<(usize, usize), Vec<usize>> = BTreeMap::new();
    for i in 1..n + 1 {
        let key = parse_first_last(&i);
        if key.0 == 0 || key.1 == 0 {
            continue;
        }
        if m.contains_key(&key) {
            let v = m.get_mut(&key).unwrap();
            v.push(i);
        } else {
            m.insert(key, vec![i]);
        }
    }

    let result = {
        let mut sum = 0;
        for (&(first, last), v1) in m.iter() {
            if let Some(v2) = m.get(&(last, first)) {
                sum += v1.len() * v2.len();
            }
        }
        sum
    };


    println!("{}", result);
}


fn parse_first_last(n: &usize) -> (usize, usize) {
    let last = n % 10;
    let mut n = n.clone();
    while n >= 10 {
        n /= 10;
    }
    let first = n;
    (first, last)
}
