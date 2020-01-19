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

const DIV: i64 = 1_000_000_000 + 7;

fn main() {
    input! {
        n: i64,
        aa: [i64; n],
    }

    let result = {
        let mut x = aa[0];
        for &a in aa.iter().skip(1) {
            // println!("x: {}, sum: {}", x, sum);
            let g = gcd(x, a);
            let g2 = a / g;
            // sum = (sum * g2) % DIV;
            x = x * g2;
            // let plus = x / a;
            // sum = (sum + plus) % DIV;
            // println!("x: {}, sum: {}", x, sum);
            // let g = gcd(x, a) % DIV;
            // // let g2 = a / g;
            // let g2 = (a * modinv(g, DIV)) % DIV;
            // sum = (sum * g2) % DIV;
            // x = (x * g2) % DIV;
            // let plus = (x * modinv(a, DIV)) % DIV;
            // sum = (sum + plus) % DIV;
        }
        // println!("x: {},", x);
        x = x % DIV;
        let mut sum = 0;
        for a in aa {
            let plus = (x * modinv(a, DIV)) % DIV;
            sum = (sum + plus) % DIV;
        }
        // TODO
        sum
    };

    println!("{}", result);
}

fn gcd(l: i64, r: i64) -> i64 {
    if r == 0 {
        l
    } else {
        gcd(r, l % r)
    }
}

fn modinv(a: i64, m: i64) -> i64 {
    let mut a = a;
    let mut b = m;
    let mut u = 1;
    let mut v = 0;
    while b != 0 {
        let t = a / b;
        a -= t * b;
        {
            let tmp = b;
            b = a;
            a = tmp;
        }
        u -= t * v;
        {
            let tmp = v;
            v = u;
            u = tmp;
        }
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    return u;
}
