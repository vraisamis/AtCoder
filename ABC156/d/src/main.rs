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
        a: i64,
        b: i64,
    }

    let mut sum = 0;
    // let mut u = (1..n + 1).into_iter().fold(1, |cum, x| (cum * x) % DIV);
    // let mut r = 1;
    // let mut u = 1;
    // let mut r = 1;
    // println!("u: {}, r: {}, k: {}, sum: {}  (a: {}, b: {})", u, r, 0, sum, a, b);
    let mut t = 1;
    for k in (1..n + 1).into_iter() {
        t *= 1 + n - k;
        t %= DIV;
        t *= modinv(k, DIV) % DIV;
        t %= DIV;
        if k != a && k != b {
            // println!("        tmp: {}", tmp);
            sum += t;
            sum %= DIV;
        }
        // u *= 1 + n - k;
        // u %= DIV;
        // r *= k;
        // r %= DIV;
        // // println!("u: {}, r: {}, k: {}, sum: {}  (a: {}, b: {})", u, r, k, sum, a, b);
        // if k != a && k != b {
        //     let tmp = (u * modinv(r, DIV)) % DIV;
        //     // println!("        tmp: {}", tmp);
        //     sum += tmp;
        //     sum %= DIV;
        // }
    }

    println!("{}", sum);
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn modinv_test() {
        let a = 12345678900000;
        let b = 100000;
        let a = a % DIV;
        let result = a * modinv(b, DIV) % DIV;
        assert_eq!(result, 123456789);
    }
}
