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
        a: i64,
        b: i64,
        c: i64,
        ll: [i64; n]
    }
    let rule = Rule {
        n: n,
        a: a,
        b: b,
        c: c,
        ll: ll
    };
    let result = dfs(&rule, 0, 0, 0, 0).unwrap();
    println!("{}", result - 30);
}

struct Rule {
    n: usize,
    a: i64,
    b: i64,
    c: i64,
    ll: Vec<i64>
}

fn dfs(rule: &Rule, cur: usize, a: i64, b: i64, c: i64) -> Option<i64> {
    if cur == rule.n {
        if vec![a, b, c].into_iter().min().unwrap() == 0 {
            return None;
        }
        let ad = (a - rule.a).abs();
        let bd = (b - rule.b).abs();
        let cd = (c - rule.c).abs();
        return Some(ad + bd + cd);
    }
    vec![
        dfs(rule, cur + 1, a, b, c),
        dfs(rule, cur + 1, a + rule.ll[cur], b, c).map(|v| v + 10),
        dfs(rule, cur + 1, a, b + rule.ll[cur], c).map(|v| v + 10),
        dfs(rule, cur + 1, a, b, c + rule.ll[cur]).map(|v| v + 10),
    ].into_iter()
        .filter_map(|v| v).min()
}
