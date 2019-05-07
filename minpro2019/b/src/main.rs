// 日本語

use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let mut vsa: Vec<u32> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();

    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let vs: Vec<u32> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
    vsa.push(vs[0]);
    vsa.push(vs[1]);

    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let vs: Vec<u32> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
    vsa.push(vs[0]);
    vsa.push(vs[1]);

    let condvec = vec![
        vsa.iter().filter(|x| {**x == 1}).count(),
        vsa.iter().filter(|x| {**x == 2}).count(),
        vsa.iter().filter(|x| {**x == 3}).count(),
        vsa.iter().filter(|x| {**x == 4}).count()
    ];
    let cond =
        condvec.iter().filter(|x| {**x == 1}).count() == 2
        && condvec.iter().filter(|x| {**x == 1}).count() == 2;
    if cond {
        println!("YES");
    } else {
        println!("NO");
    }
}
