use std::collections::HashMap;

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
    input! {
        n: usize,
        q: usize,
        s: chars,
        td: [(String, String); q]
    };
    let mut lcurrent = 0;
    let mut rcurrent = 0;
    for &(ref t, ref d) in td.iter().rev() {
        let t = t.chars().next().unwrap();
        let d = d.chars().next().unwrap();
        if d == 'L' {
            if lcurrent < n && s[lcurrent] == t {
                lcurrent += 1;
            }
            if rcurrent > 0 && s[n - rcurrent] == t {
                rcurrent -= 1;
            }
        } else {
            if rcurrent < n && s[n - rcurrent - 1] == t {
                rcurrent += 1;
            }
            if lcurrent > 0 && s[lcurrent - 1] == t {
                lcurrent -= 1;
            }
        }
    }

    println!("{}", n - lcurrent - rcurrent);
}
