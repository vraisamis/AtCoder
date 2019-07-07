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
        w: usize,
        h: usize,
        x: usize,
        y: usize,
    }
    let min = w * h / 2;
    let odd = w % 2 == 1 && h % 2 == 1;
    let ww = vec![x, w - x];
    let hh = vec![y, h - y];
    // center
    let cnt_result = if ww[0] == ww[1] && hh[0] == hh[1] {
        1
    } else {
        0
    };
    // let rects = vec![
    //     vec![x * y, x * (h - y),],
    //     vec![(w - x) * y, (w - x) * (h - y),],
    // ];
    // // println!("{:?}", rects);
    // let hol = vec![rects[0][0] + rects[0][1], rects[1][1] + rects[1][0],];
    // let hol = hol.iter().min().unwrap();
    // let vert = vec![rects[0][0] + rects[1][0], rects[1][1] + rects[0][1],];
    // let vert = vert.iter().min().unwrap();
    // let sum_rects = vec![
    //     hol,
    //     vert
    // ];
    // // println!("{:?}", sum_rects);
    // let min = sum_rects.iter().max().unwrap();
    // let cnt = sum_rects.iter().filter(|&v| v == min).count();
    // let cnt_result = if cnt > 1 {
    //     1
    // } else {
    //     0
    // };
    println!("{}{} {}", min, if odd { ".5" } else { "" }, cnt_result);
}
