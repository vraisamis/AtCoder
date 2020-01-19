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

macro_rules! order_by {
    ($($x:tt)*) => {
        |l, r| {
            order_by_inner!(l, r, $($x)*)
        }
    }
}
macro_rules! order_by_inner {
    () => {};
    ($l:ident) => {std::cmp::Ordering::Equal};
    ($l:ident , ) => {std::cmp::Ordering::Equal};
    ($l:ident , $r:ident) => {std::cmp::Ordering::Equal};
    ($l:ident , $r:ident , ) => {std::cmp::Ordering::Equal};

    // last
    ($l:ident , $r:ident , $x:tt asc) => {
        $l.$x.cmp(&$r.$x)
    };
    ($l:ident , $r:ident , $x:tt desc) => {
        $l.$x.cmp(&$r.$x).reverse()
    };
    ($l:ident , $r:ident , $x:tt) => {
        order_by_inner!($l, $r, $x asc)
    };

    // mid
    ($l:ident , $r:ident , $x:tt asc , $($p:tt)+) => {
        match $l.$x.cmp(&$r.$x) {
            std::cmp::Ordering::Equal => {
                order_by_inner!($l, $r, $($p)+)
            },
            other => other
        }
    };
    ($l:ident , $r:ident , $x:tt desc , $($p:tt)+) => {
        match $l.$x.cmp(&$r.$x).reverse() {
            std::cmp::Ordering::Equal => {
                order_by_inner!($l, $r, $($p)+)
            },
            other => other
        }
    };
    ($l:ident , $r:ident , $x:tt , $($p:tt)+) => {
        order_by_inner!($l, $r, $x asc, $($p)+)
    };
}
fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
    }

    let mut ans = -1;
    // 0b0000 A3, A2, A1, A0
    // 0b0101 A3, B2, A1, B0
    for bit in 0b0 .. 0b1 << n {
        let mut even_pos = vec![];
        let mut odd_pos = vec![];

        // use cards
        for i in 0..n {
            let i_is_even = i % 2 == 0;
            if bit & (0b1 << i) == 0 {
                // use A
                if i_is_even {
                    even_pos.push((aa[i], i));
                } else {
                    odd_pos.push((aa[i], i));
                }
            } else {
                // use B
                if i_is_even {
                    odd_pos.push((bb[i], i));
                } else {
                    even_pos.push((bb[i], i));
                }
            }
        }

        // cannot create
        let ok = if n % 2 == 0 {
            even_pos.len() == odd_pos.len()
        } else {
            even_pos.len() == odd_pos.len() + 1
        };
        if !ok {
            continue;
        }

        even_pos.sort();
        odd_pos.sort();

        let result = {
            let mut v = vec![];
            let mut ei = even_pos.into_iter();
            let mut oi = odd_pos.into_iter();
            loop {
                match (ei.next(), oi.next()) {
                    (Some(e), Some(o)) => {
                        v.push(e);
                        v.push(o);
                    },
                    (Some(e), None) => {
                        v.push(e);
                    },
                    (None, Some(o)) => {
                        v.push(o);
                    },
                    (None, None) => break
                }
            }
            v
        };
        if !sorted(&result.iter().map(|r| r.0).collect::<Vec<_>>()) {
            continue;
        }

        let n = bubble_cnt(&result, &|r| r.1) as isize;
        if ans == -1 || ans > n {
            ans = n;
        } else {
        }
    }

    println!("{}", ans);
}

fn sorted<T>(v: &[T]) -> bool
where
    T: Ord
{
    v.windows(2).all(|w| w[0] <= w[1])
}

fn bubble_cnt<T, F>(v: &Vec<T>, f: &F) -> usize
where
    F: Fn(&T) -> usize
{
    let mut indices = v.iter().map(f).collect::<Vec<_>>();
    let mut cnt = 0;
    let mut last_change = indices.len() - 1;
    while last_change > 1 {
        let mut tmp = 0;
        for i in 0..last_change {
            if indices[i] > indices[i + 1] {
                cnt += 1;
                indices.swap(i, i + 1);
                tmp = i + 1;
            }
        }
        last_change = tmp;
    }
    cnt
}
