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
        _n: usize,
        cs: String
    }
    let leftw = cs.chars()
        .skip_while(|&c| c == '.')
        .collect::<String>();
    // println!("{}", leftw);
    let rightb = leftw.chars().rev()
        .skip_while(|&c| c == '#')
        .collect::<String>();
    // println!("{}", rightb);
    // 右からblackで塗るコスト
    let mut vb = vec![0; rightb.len() + 1];
    // 右端は白になっていることはわかっている
    for (i, c) in rightb.chars().enumerate() {
        if c == '.' {
            vb[i + 1] = vb[i] + 1;
        } else {
            vb[i + 1] = vb[i];
        }
    }
    // println!("vb: {:?}", vb.iter().rev().collect::<Vec<&usize>>());
    // let vb = vb.iter().rev().collect

    // 左からwhiteで塗るコスト
    let mut vw = vec![0; rightb.len() + 1];
    // 左端は黒になっていることはわかっている
    for (i, c) in rightb.chars().rev().enumerate() {
        if c == '#' {
            vw[i + 1] = vw[i] + 1;
        } else {
            vw[i + 1] = vw[i];
        }
    }
    // println!("vw: {:?}", vw);
    let v = vw.iter().zip(vb.iter().rev()).map(|(l, r)| l + r).min().unwrap_or(0);
    // let s = rightb.chars().rev()
    //     .collect::<String>();
    // let blacks = rightb.chars()
    //     .filter(|&c| c == '#')
    //     .count();
    // println!("{}", blacks);
    // let whites = rightb.chars()
    //     .filter(|&c| c == '.')
    //     .count();
    // println!("{}", whites);
    // let result = std::cmp::min(blacks, whites);
    println!("{}", v);
}
