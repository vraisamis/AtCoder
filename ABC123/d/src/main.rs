use std::collections::BinaryHeap;
use std::collections::BTreeSet;
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

    #[allow(unused_macros)]
    macro_rules! to_mut {
        ($($var: ident),*) => {
            $(
                #[allow(unused_mut)]
                let mut $var = $var;
            )*
        };
    }
fn main() {
    input!{
        x: usize,
        y: usize,
        z: usize,
        k: usize,
        aa: [usize; x],
        bb: [usize; y],
        cc: [usize; z]
    }
    to_mut!(aa, bb, cc);
    aa.sort_by(|l, r| r.cmp(l));
    bb.sort_by(|l, r| r.cmp(l));
    cc.sort_by(|l, r| r.cmp(l));
    // println!("done1");
    // let mut v = vec![0; x * y * z];
    // for (i, a) in aa.iter().enumerate() {
    //     for (j, b) in bb.iter().enumerate() {
    //         for (k, c) in cc.iter().enumerate() {
    //             v[i * (y * z) + j * z + k] = a + b + c;
    //         }
    //     }
    // }
    // println!("done2");
    let mut h: BinaryHeap<V> = BinaryHeap::new();
    let mut s = BTreeSet::new();
    h.push(V::new(0, 0, 0, aa[0] + bb[0] + cc[0]));
    s.insert(V::new(0, 0, 0, aa[0] + bb[0] + cc[0]));
    for i in  0..k {
        let v = h.pop().unwrap();
        println!("{}", v.v);
        if v.a + 1 < x {
            let va = V::new(v.a + 1, v.b, v.c, aa[v.a + 1] + bb[v.b] + cc[v.c]);
            if !s.contains(&va) {
                h.push(va.clone());
                s.insert(va.clone());
            }
        }
        if v.b + 1 < y {
            let vb = V::new(v.a, v.b + 1, v.c, aa[v.a] + bb[v.b + 1] + cc[v.c]);
            if !s.contains(&vb) {
                h.push(vb.clone());
                s.insert(vb.clone());
            }
        }
        if v.c + 1 < z {
            let vc = V::new(v.a, v.b, v.c + 1, aa[v.a] + bb[v.b] + cc[v.c + 1]);
            if !s.contains(&vc) {
            h.push(vc.clone());
            s.insert(vc.clone());
            }
        }
    }
}

#[derive(Eq, Clone)]
struct V {
    a: usize,
    b: usize,
    c: usize,
    v: usize
}

impl std::cmp::PartialEq for V {
    fn eq(&self, other: &V) -> bool {
        self.a == other.a &&
            self.b == other.b &&
            self.c == other.c &&
            self.v == other.v
    }
}

impl std::cmp::PartialOrd for V {
    fn partial_cmp(&self, other: &V) -> Option<std::cmp::Ordering>{
        Some(match self.v.cmp(&other.v) {
            std::cmp::Ordering::Equal => {
                match self.a.cmp(&other.a) {
                    std::cmp::Ordering::Equal => {
                        match self.b.cmp(&other.b) {
                            std::cmp::Ordering::Equal => {
                                self.c.cmp(&other.c)
                            },
                            other => other
                        }
                    },
                    other => other
                }
            },
            other => other
        })
    }
}
impl std::cmp::Ord for V {
    fn cmp(&self, other: &V) -> std::cmp::Ordering {
        match self.v.cmp(&other.v) {
            std::cmp::Ordering::Equal => {
                match self.a.cmp(&other.a) {
                    std::cmp::Ordering::Equal => {
                        match self.b.cmp(&other.b) {
                            std::cmp::Ordering::Equal => {
                                self.c.cmp(&other.c)
                            },
                            other => other
                        }
                    },
                    other => other
                }
            },
            other => other
        }
    }
}
impl V {
    fn new(a: usize, b: usize, c: usize, v: usize) -> V {
        V {a: a, b: b, c: c, v: v}
    }
}
