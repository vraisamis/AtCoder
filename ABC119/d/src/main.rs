#[allow(unused_imports)]
use std::cmp::min;
#[allow(unused_imports)]
use std::cmp::max;

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
        an: usize,
        bn: usize,
        qn: usize,
        aa: [i64; an],
        bb: [i64; bn],
        qq: [i64; qn]
    }
    for q in qq {
        let ai = lower_bound(&aa, &q);
        let bi = lower_bound(&bb, &q);
        let mut a_list = vec![];
        // east
        if ai < an {
            a_list.push(aa[ai] - q);
        }
        // west
        if ai > 0 {
            a_list.push(aa[ai - 1] - q);
        }
        let mut b_list = vec![];
        // east
        if bi < bn {
            b_list.push(bb[bi] - q);
        }
        // west
        if bi > 0 {
            b_list.push(bb[bi - 1] - q);
        }
        let mut minimum = 1000000000000i64;
        for &a in a_list.iter() {
            for &b in b_list.iter() {
                // 逆方向
                let m = if a > 0 && b < 0 || a < 0 && b > 0 {
                    a.abs() + b.abs() + min(a.abs(), b.abs())
                } else {
                    max(a.abs(), b.abs())
                };
                minimum = min(minimum, m);
            }
        }
        println!("{}", minimum);

    }
}

#[allow(dead_code)]
fn lower_bound<T>(v: &Vec<T>, x: &T) -> usize 
where T: Ord {
    let mut left = 0;
    let mut right = v.len();
    while left < right {
        let mid = (left + right) / 2;
        let value = &v[mid];
        if x <= value {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    return right;
}

#[allow(dead_code)]
fn upper_bound<T>(v: &Vec<T>, x: &T) -> usize 
where T: Ord {
    let mut left = 0;
    let mut right = v.len();
    while left < right {
        let mid = (left + right) / 2;
        let value = &v[mid];
        if x < value {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    return left;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn lb() {
        let a = vec![10, 11, 13, 13, 15];
        assert_eq!(lower_bound(&a, &9), 0);
        assert_eq!(lower_bound(&a, &10), 0);
        assert_eq!(lower_bound(&a, &11), 1);
        assert_eq!(lower_bound(&a, &12), 2);
        assert_eq!(lower_bound(&a, &13), 2);
        assert_eq!(lower_bound(&a, &14), 4);
        assert_eq!(lower_bound(&a, &15), 4);
        assert_eq!(lower_bound(&a, &16), 5);
    }

    #[test]
    fn ub() {
        let a = vec![10, 11, 13, 13, 15];
        assert_eq!(upper_bound(&a, &9), 0);
        assert_eq!(upper_bound(&a, &10), 1);
        assert_eq!(upper_bound(&a, &11), 2);
        assert_eq!(upper_bound(&a, &12), 2);
        assert_eq!(upper_bound(&a, &13), 4);
        assert_eq!(upper_bound(&a, &14), 4);
        assert_eq!(upper_bound(&a, &15), 5);
        assert_eq!(upper_bound(&a, &16), 5);
    }
}
