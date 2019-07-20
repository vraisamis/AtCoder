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
        aa: [f64; n],
    }

    // 準備
    let mut mat = vec![vec![0f64; n]; n];
    for i in 0..n {
        mat[i][i] = 1f64 / 2f64;
        mat[i][(i + 1) % n] = 1f64 / 2f64;
    }

    lu(&mut mat);
    let mut aa2 = vec![0f64; n];
    // 全身
    for i in 0..n {
        let mut sum = 0f64;
        for j in 0 .. i {
            sum += aa2[j] * mat[i][j];
        }
        let a = aa[i] - sum;
        let v = a / mat[i][i];
        aa2[i] = v;
    }
    let mut bb = vec![0f64; n];
    // 交代
    for i in (0..n).rev() {
        let mut sum = 0f64;
        for j in i + 1 .. n {
            sum += bb[j] * mat[i][j];
        }
        let b = aa2[i] - sum;
        bb[i] = b;
    }
    print!("{}", bb[0]);
    for i in 1..n {
        print!(" {}", bb[i]);
    }
    println!("");
}

fn lu(mat: &mut Vec<Vec<f64>>) -> Option<()> {
    let n = mat.len();
    for k in 0..n {
        for i in k..n {
            let mut sum = 0f64;
            for j in 0..k {
                sum += mat[i][j] * mat[j][k];
            }
            mat[i][k] -= sum;
        }
        if mat[k][k] == 0f64 {
            return None;
        }

        let tmp = 1f64 / mat[k][k];
        for j in k + 1 .. n {
            let mut sum = 0f64;
            for i in 0..k {
                sum += mat[k][i] * mat[i][j];
            }
            mat[k][j] = (mat[k][j] - sum) * tmp;
        }
    }
    Some(())
}
