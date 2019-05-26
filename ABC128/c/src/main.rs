#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

#[allow(dead_code)]
fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

#[allow(dead_code)]
fn readvv<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| readv()).collect()
}

fn main() {
    let (n, m) = {
        let tv = readv();
        (tv[0], tv[1])
    };
    // println!("n: {}, m: {}", n, m);

    let mut ss = vec![];
    for _ in 0..m {
        let s = readv::<usize>();
        let s = s.into_iter().skip(1).map(|i| i - 1).collect::<Vec<usize>>();
        ss.push(s);
    }

    // println!("ss: ");
    // for s in ss {
    //     println!("{:?}", s);
    // }
    // println!("----");

    let pp = readv::<usize>();

    // println!("{:?}", pp);

    let pf = |ls, i| {
        let mut count = 0;
        for sv in ls {
            // println!("        sv: {}, i: {}", sv, i);
            if i & (0x1 << sv) > 0 {
                // println!("          OKOK");
                count += 1;
            }
        }
        count % 2
    };
    // zentansaku
    let mut count = 0;
    for i in 0..(1 << n) {
        let ok = ss.iter().enumerate().all(|(l, s)| {
            // println!("i: {} , l: {}, s: {:?}", i, l, s);
            let pv = pf(s, i);
            let p = pp[l];
            // println!("  (pv: {}, p: {})", pv, p);
            pv == p
        });
        if ok {
            count += 1;
        }
    }
    println!("{}", count);
}
