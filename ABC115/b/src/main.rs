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

fn main() {
    let n: usize = read();
    let mut p: Vec<usize> = Vec::new();
    for _ in 0..n {
        p.push(read());
    }
    p.sort();
    println!("{}", p.iter().fold(0, |sum, i| sum + i) - p[n - 1] / 2);
}
