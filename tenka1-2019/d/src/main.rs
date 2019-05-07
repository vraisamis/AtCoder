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

const MOD: usize = 998244353;
fn main() {
    input!{
        n: usize,
        aa: [usize; n]
    }

    let mut aa = aa;
    aa.sort();
    let aa = aa.into_iter().rev().collect::<Vec<usize>>();
    // println!("{:?}", aa);
    let sum: usize = aa.iter().fold(0, |sum, a| sum + a);
    // 後ろからの累積和
    let mut cumsum = vec![sum; n];
    for (i, value) in aa.iter().enumerate() {
        if i + 1 >= aa.len() {
            break;
        }
        cumsum[i + 1] = cumsum[i] - *value;
    }
    // println!("{:?}", cumsum);
    // たぶんメモ化が必要？
    let result = rec(&aa, &cumsum, 0, 0, 0, 0);
    println!("{}", result);
}


// return (通り, rsum, gsum, bsum)
#[allow(dead_code)]
fn rec(aa: &Vec<usize>, cumsum: &Vec<usize>, r: usize, g: usize, b: usize, k: usize) -> usize {
    let n = aa.len();
    let mut v = vec![r, g, b];
    v.sort();
    if k >= n {
        // 三角不等式
        if v[2] < v[1] + v[0] {
            return 1;
        } else {
            return 0;
        }
    }
    if v[2] + cumsum[k] < v[1] + v[0] {
        let v = (3i64.pow((n - k) as u32) as usize) % MOD;
        // println!("r: {}, g: {}, b: {}, cumsum: {:?}, k: {}, v: {}", r, g, b, cumsum, k, v);
        return v;
    }
    // r
    let result = rec(aa, cumsum, r + aa[k], g, b, k + 1);
    let result = (result + rec(aa, cumsum, r, g + aa[k], b, k + 1)) % MOD;
    let result = (result + rec(aa, cumsum, r, g, b + aa[k], k + 1)) % MOD;

    return result;
}
