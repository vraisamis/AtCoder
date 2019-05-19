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
        mut edges: [(usize1, usize1, usize); n - 1]
    }
    let mut e2 = edges.iter().map(|&(u, v, w)| (v, u ,w)).collect();
    edges.append(&mut e2);
    edges.sort_by(ord);

    // kuro
    // println!("{:?}", edges);
    let mut tbl = vec![None; n];
    // println!("{:?}", tbl);
    // to white
    paint(&mut tbl, &edges, 0, 0);
    // println!("{:?}", tbl);

    for v in tbl {
        println!("{}", v.unwrap());
    }
}

fn ord(l: &(usize, usize, usize), r: &(usize, usize, usize)) -> std::cmp::Ordering {
    let lu = l.0;
    let ru = r.0;
    lu.cmp(&ru)
}

fn paint(table: &mut Vec<Option<u8>>, edges: &Vec<(usize, usize, usize)>, i: usize, v: u8) {
    const VS: [u8; 2] = [1, 0];
    // println!("i: {}", i);
    table[i] = Some(v);
    let begin = lower_bound_by(edges, &(i, 0, 0), ord);
    let end = upper_bound_by(edges, &(i, 0, 0), ord);
    for k in begin..end {
        let (_current, next, weight) = edges[k];
        // println!("  edges[{}]: current: {}, next: {}, weight: {}", k, _current, next, weight);
        if table[next].is_some() {
            continue;
        }
        if weight % 2 == 0 {
            paint(table, edges, next, v);
        } else {
            paint(table, edges, next, VS[v as usize]);
        }
    }
}

#[allow(dead_code)]
fn lower_bound_by<T, F>(v: &Vec<T>, x: &T, f: F) -> usize 
where
T: Ord,
F: Fn(&T, &T) -> std::cmp::Ordering {
    let mut left = 0;
    let mut right = v.len();
    while left < right {
        let mid = (left + right) / 2;
        let value = &v[mid];
        let cmp = f(x, value);
        if cmp == std::cmp::Ordering::Greater {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return right;
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
fn upper_bound_by<T, F>(v: &Vec<T>, x: &T, f: F) -> usize 
where
T: Ord,
F: Fn(&T, &T) -> std::cmp::Ordering {
    let mut left = 0;
    let mut right = v.len();
    while left < right {
        let mid = (left + right) / 2;
        let value = &v[mid];
        let cmp = f(x, value);
        if cmp == std::cmp::Ordering::Less {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    return left;
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
