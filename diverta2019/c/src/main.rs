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
        ss: [String; n]
    }
    let firstc = |s: &String| s.chars().next();
    let lastc = |s: &String| s.chars().rev().next();
    let cnt_ab = |s: &String| {
        let mut lastchar = '_';
        let mut cnt = 0;
        for c in s.chars() {
            if lastchar == 'A' && c == 'B' {
                cnt += 1;
            }
            lastchar = c;
        }
        cnt
    };
    let mut cnt: usize = 0;
    let mut first_is_b = 0;
    let mut last_is_a = 0;
    let mut ba = 0;
    for s in ss {
        cnt += cnt_ab(&s);
        let f = firstc(&s);
        let fb = f.is_some() && f.unwrap() == 'B';
        let l = lastc(&s);
        let la = l.is_some() && l.unwrap() == 'A';
        match (fb, la) {
            (true, true) => {
                ba += 1;
            },
            (true, false) => {
                first_is_b += 1;
            },
            (false, true) => {
                last_is_a += 1;
            },
            _ => ()
        }
    }
    let result = cnt + ba + vec![first_is_b, last_is_a].into_iter().min().unwrap();
    let result = if ba > 0 && last_is_a == 0 && first_is_b == 0 {
        result - 1
    } else {
        result
    };
    println!("{}", result);
}
