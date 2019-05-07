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
        q: usize,
        s: String,
        lr: [(usize1, usize1); q]
    }


    let matches: Vec<usize> = s.match_indices("AC").map(|(i, _)| i).collect();
    let mut bit = BIT::new(n);
    for k in matches {
        bit.add(k, 1);
    }
    for (l, r) in lr {
        println!("result: {}", bit.sum(l, r - 1));
    }
}

// struct BIT {
//     v: Vec<usize>,
//     n: usize
// }
//
// impl BIT {
//     #![allow(dead_code)]
//     fn new(n: usize) -> BIT {
//         let v = vec![0; n];
//         BIT { v: v, n: n }
//     }
//
//     fn from(v: Vec<usize>) -> BIT {
//         let mut bit = BIT::new(v.len());
//         for (i, value) in v.iter().enumerate() {
//             bit.add(i, *value);
//         }
//         bit
//     }
//
//     fn add(&mut self, index: usize, value: usize) {
//         let mut x = index + 1;
//         while x <= self.n {
//             self.v[x - 1] += value;
//             x += ((x as i64) & -(x as i64)) as usize;
//         }
//     }
//
//     fn sum(&self, left: usize, right: usize) -> usize {
//         self.sum0(right) - if left > 0 { self.sum0(left - 1) } else { 0 }
//     }
//
//     fn sum0(&self, right: usize) -> usize {
//         let mut sum = 0;
//         let mut x = right + 1;
//         while x > 0 {
//             sum += self.v[x - 1];
//             x -= ((x as i64) & -(x as i64)) as usize;
//         }
//         sum
//     }
// }

struct BIT<T>
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::default::Default + std::clone::Clone + std::marker::Copy
{
    v: Vec<T>,
    n: usize
}

impl <T> BIT<T>
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::default::Default + std::clone::Clone + std::marker::Copy
{
    #![allow(dead_code)]
    fn new(n: usize) -> BIT<T> {
        let v = vec![T::default(); n];
        BIT { v: v, n: n }
    }

    fn from(v: Vec<T>) -> BIT<T> {
        let mut bit = BIT::new(v.len());
        for (i, value) in v.iter().enumerate() {
            bit.add(i, *value);
        }
        bit
    }

    fn add(&mut self, index: usize, value: T) {
        let mut x = index + 1;
        while x <= self.n {
            self.v[x - 1] = self.v[x - 1] + value;
            x += ((x as i64) & -(x as i64)) as usize;
        }
    }

    fn sum(&self, left: usize, right: usize) -> T {
        self.sum0(right) - if left > 0 { self.sum0(left - 1) } else { T::default() }
    }

    fn sum0(&self, right: usize) -> T {
        let mut sum = T::default();
        let mut x = right + 1;
        while x > 0 {
            sum = sum + self.v[x - 1];
            x -= ((x as i64) & -(x as i64)) as usize;
        }
        sum
    }
}


#[allow(dead_code)]
fn cumlative_sum<T>(v: Vec<T>) -> Vec<T>
where
    T: std::ops::Add<Output = T> + std::clone::Clone + std::marker::Copy
{
    let n = v.len();
    if n == 0 {
        return Vec::new();
    }
    let mut result = vec![v[0]; n];
    for (i, value) in v.iter().enumerate().skip(1) {
        result[i] = result[i - 1] + *value;
    }
    result
}
