// 日本語
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
        m: usize,
        bridges: [[usize; 2]; m]
    }
    // let vs: Vec<usize> = readv();
    // let (n, m) = (vs[0], vs[1]);

    // let bridges: Vec<Vec<usize>> = readvv(m);

    let mut uni = UnionFind::new(n + 1);
    let maximum = n * (n - 1) / 2;
    let mut results: Vec<u64> = vec![maximum as u64; m + 1];
    for (i, bridge) in bridges.iter().skip(1).enumerate().rev() {
        let (left, right) = (bridge[0], bridge[1]);
        results[i] = results[i + 1];
        if !uni.same(left, right) {
            let (lsize, rsize) = (uni.size(left) as u64, uni.size(right) as u64);
            uni.unite(left, right);
            results[i] -= lsize * rsize;
        }
    }

    for i in results.iter().take(m) {
        println!("{}", i);
    }
}

struct UnionFind {
    parent: Vec<isize>
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: vec![-1; n]
        }
    }

    fn root(&mut self, i: usize) -> usize {
        let par = self.parent[i];
        if par < 0 {
            i
        } else {
            let res = self.root(par as usize);
            self.parent[i] = res as isize;
            res
        }
    }

    fn same(&mut self, l: usize, r: usize) -> bool {
        self.root(l) == self.root(r)
    }

    fn unite(&mut self, l:usize, r:usize) -> bool {
        if self.same(l, r) {
            return false;
        }
        let u = |s: &mut UnionFind, p, c| {
            let proot = s.root(p);
            let croot = s.root(c);
            s.parent[proot] += s.parent[croot];
            s.parent[croot] = p as isize;
        };
        if self.size(l) < self.size(r) {
            u(self, r, l);
            // let (l, r) = (r, l);
        } else {
            u(self, l, r);
        }
        true
    }

    fn size(&mut self, i: usize) -> usize {
        let r = self.root(i);
        -self.parent[r as usize] as usize
    }
}

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

