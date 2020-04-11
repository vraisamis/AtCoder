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

// #[derive(Debug)]
// enum Query {
//     Reverse,
//     Add(usize, String),
// }

fn main() {
    let s: String = read();
    let qn: usize = read();
    // let mut qq: Vec<Query> = vec![];

    let mut head: String = "".into();
    let mut tail: String = "".into();
    let mut reversed = false;
    for _ in 0..qn {
        // get query
        let tmp: String = read();
        let mut it = tmp.split_whitespace();
        let qtype: u64 = it.next().map(|v| v.parse().ok().unwrap()).unwrap();
        if qtype == 1 {
            reversed = !reversed;
        } else {
            let add_to_head = it.next().map(|v| v.parse::<u8>().ok().unwrap()).unwrap() == 1;
            let s = it.next().unwrap().to_string();
            let s = if reversed {
                s.chars().rev().collect()
            } else {
                s
            };

            match (reversed, add_to_head) {
                (false, true) => {
                    head = s + &head;
                },
                (true, true) => {
                    tail = tail + &s;
                },
                (false, false) => {
                    tail = tail + &s;
                },
                (true, false) => {
                    head = s + &head;
                }
            }
        }
        // qq.push(query);
    }

    let rtmp = head + &s + &tail;
    let result = if reversed {
        rtmp.chars().rev().collect()
    } else {
        rtmp
    };
    println!("{}", result);
    // println!("{:?}", qq);

}
        // let tmp: String = read();
        // let mut it = tmp.split_whitespace();
        // let qtype: u64 = it.next().map(|v| v.parse().ok().unwrap()).unwrap();
        // let query = if qtype == 1 {
        //     Query::Reverse
        // } else {
        //     let orient: usize = it.next().map(|v| v.parse().ok().unwrap()).unwrap();
        //     Query::Add(orient, it.next().unwrap().to_string())
        // };
