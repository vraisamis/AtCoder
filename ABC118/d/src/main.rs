// 日本語
use std::io::stdin;
use std::cmp::{PartialOrd, Ordering};

fn main() {
    let stick = [99, 2, 5, 5, 4, 5, 6, 3, 7, 6];
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let vs: Vec<u16> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();
    let n = vs[0];

    s = String::new();
    stdin().read_line(&mut s).ok();
    let vs: Vec<u16> = s.trim().split_whitespace().map(|token| token.parse().ok().unwrap()).collect();

    let mut a_stick: Vec<(u16, u16)> = stick
        .iter()
        .enumerate()
        .map(|(i, &xi)| { if vs.contains(&(i as u16)) { (i as u16, xi) } else { (i as u16, 99) } })
        .filter(|&(_, xi)| { xi <= 7 })
        .collect();
    a_stick.sort_by(|&(li, lxi), &(ri, rxi)| {
        match lxi.partial_cmp(&rxi).unwrap() {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => li.partial_cmp(&ri).unwrap(),
        }
    });

    
}
