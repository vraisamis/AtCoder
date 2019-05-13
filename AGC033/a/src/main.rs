use std::collections::VecDeque;

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
        h: usize,
        w: usize,
        board: [chars; h]
    }
 
    let mut tbl = vec![vec![None; w]; h];
    let mut queue = VecDeque::new();
    for j in 0..h {
        for i in 0..w {
            // kuro
            if board[j][i] == '#' {
                queue.push_back((j, i, 0));
            }
        }
    }
 
    while let Some((j, i, v)) = queue.pop_front() {
        if tbl[j][i].is_some() {
            continue;
        }
        tbl[j][i] = Some(v);
        for &(jj, ii) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let j = j as i64 + jj;
            let i = i as i64 + ii;
            if j < 0 || i < 0 {
                continue;
            }
            let j = j as usize;
            let i = i as usize;
            if j >= h || i >= w {
                continue;
            }
            queue.push_back((j, i, v + 1));
        }
    }

    let result = tbl.iter().filter_map(|line| line.iter().filter_map(|v| *v).max()).max().unwrap();
    println!("{}", result);
}
