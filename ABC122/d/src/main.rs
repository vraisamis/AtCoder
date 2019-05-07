use std::collections::HashMap;

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

const MOD: usize = 1_000_000_000 + 7;

fn main() {
    input!{
        n: usize
    }
    let mut table = vec![HashMap::new(); n];
    let result = dfs(&mut table, 0, n, "TTT".to_string());
    println!("{}", result);
}

fn dfs(table: &mut Vec<HashMap<String, usize>>, current: usize, n: usize, last3: String) -> usize {
    if current == n {
        return 1;
    }
    if table[current].contains_key(&last3) {
        return table[current][&last3];
    }
    let mut result = 0;
    for c in "ACGT".chars() {
        let ns = last3.clone() + &(c.to_string());
        if ok(&ns) {
            let last3 = ns.chars().skip(1).collect();
            result = (result + dfs(table, current + 1, n, last3)) % MOD;
        }
    }
    table[current].insert(last3, result);
    result
}

fn ok(last4: &String) -> bool {
    let chars = last4.chars().collect::<Vec<char>>();
    for i in 0..4 {
        let s = if i >= 1 {
            let mut tmp = chars.clone();
            let t = tmp[i];
            tmp[i] = tmp[i - 1];
            tmp[i - 1] = t;
            tmp.into_iter().collect::<String>()
        } else {
            chars.clone().into_iter().collect::<String>()
        };
        if s.contains("AGC") {
            return false;
        }
    }
    true
}
