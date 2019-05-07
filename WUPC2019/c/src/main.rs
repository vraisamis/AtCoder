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
    // read
    input!{
        n: usize,
        m: usize,
        // zero index
        mat: [[usize1; 2]; m]
    }
    let mut edge: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for e in mat {
        let (l, r) = (e[0], e[1]);
        edge[l][r] = true;
        edge[r][l] = true;
    }

    // iから距離1で行ける2重リスト
    let mut singles: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        let mut single: Vec<usize> = Vec::new();
        for j in 0..n {
            if edge[i][j] {
                single.push(j);
            }
        }
        singles[i] = single;
    }
    for single in singles.iter() {
        println!("{:?}", single);
    }
    println!("----");
    let mut doubles: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        let mut double: Vec<usize> = Vec::new();
        for e in singles[i].iter() {
            for j in 0..n {
                if j == i {
                    continue;
                }
                if singles[i].contains(&j) {
                    continue;
                }
                if edge[*e][j] {
                    double.push(j);
                }
            }
        }
        doubles[i] = double;
    }
    for i in 0..n {
        singles[i].append(&mut doubles[i]);
    }
    for single in singles.iter() {
        println!("{:?}", single);
    }
    let mut p = P::new(singles);
    let r = p.perm(0).unwrap();
    println!("{}", r.iter().map(|v| (v + 1).to_string()).collect::<Vec<String>>().join(" "));
    // println!("{}", getperm(&singles));
}

struct P {
    list: Vec<Vec<usize>>,
    ret: Vec<usize>
}

impl P {
    fn new(list: Vec<Vec<usize>>) -> P {
        P {
            list: list,
            ret: Vec::new()
        }
    }

    fn perm(&mut self, c: usize) -> Option<Vec<usize>> {
        println!("perm called: {}", c);
        let listlen = self.list.len();
        let target = self.list[c].clone();
        for l in target.iter() {
            // retにはいってなかったらpushできる
            if self.ret.contains(&l) {
                // TODO: continue でいいかは考える
                continue;
            }
            self.ret.push(*l);
            println!("ret: {:?}", self.ret);
            if c + 1 >= listlen {
                let v = self.ret.clone();
                return Some(v);
            }
            match self.perm(c + 1) {
                Some(r) => return Some(r),
                None => {
                    self.ret.pop();
                    continue
                }
            }
        }
        None
    }
}
