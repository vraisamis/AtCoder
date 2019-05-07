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
        x: usize
    }
    // all, pat
    let mut v: Vec<(usize, usize)> = vec![(1, 1)];
    for i in 0..n {
        let (iall, ipat) = v[i];
        let all = 3 + 2 * iall;
        let pat = 1 + 2 * ipat;
        v.push((all, pat));
    }
    let mut sum = 0;
    let mut queue: Vec<(usize, usize)> = vec![(n, x)];
    while let Some((level, eat)) = queue.pop() {
        if level == 0 && eat == 1 {
            sum += 1;
            continue;
        }
        if eat <= level {
            // sum += 0;
            continue;
        }
        let (all, pat) = v[level];
        if eat == all {
            sum += pat;
            continue;
        }
        let mid = all / 2;
        match eat.cmp(&(mid + 1)) {
            std::cmp::Ordering::Less => {
                queue.push((level - 1, eat - 1));
            },
            std::cmp::Ordering::Equal => {
                let (_, pat) = v[level - 1];
                sum += pat + 1;
            },
            std::cmp::Ordering::Greater => {
                let (_, pat) = v[level - 1];
                sum += pat + 1;
                queue.push((level - 1, eat - mid - 1));
            }
        }
    }
    println!("{}", sum);
}
