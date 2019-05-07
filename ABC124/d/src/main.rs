use std::cmp::max;

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
        k: usize,
        s: chars
    }
    let mut s: Vec<usize> = s.iter().map(|&c| {
        if c == '0' {
            0
        } else {
            1
        }
    }).collect();
    let mut vs = vec![];
    let mut last = s[0];
    let mut cnt = 1;
    for i in 1..n {
        if last == s[i] {
            cnt += 1;
        } else {
            vs.push((last, cnt as usize));
            cnt = 1;
            last = s[i];
        }
    }
    vs.push((last, cnt));
    // TEST
    // println!("{:?}", vs);
    let result = if 2 * k >= vs.len() {
        vs.iter().fold(0, |sum, &(_, c)| sum + c)
    } else {
        let mut sum = 0;
        let mut m = 0;
        for (i, &(v, c)) in vs.iter().enumerate() {
            sum += c;
            if i >= 2 * k + 1 {
                sum -= vs[i - (2 * k + 1)].1;
            }
            // println!("i: {}, v: {}, c: {}, sum: {}, m: {}", i, v, c, sum, max(m, sum));
            if v == 0 {
                continue;
            }
            m = max(m, sum);
        }
        sum -= vs[vs.len() - (2 * k + 1)].1;
        max(m, sum)
    };
    println!("{}", result);
}
