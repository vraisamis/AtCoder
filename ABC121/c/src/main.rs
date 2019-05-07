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
    let vs: Vec<usize> = readv();
    let (n, m) = (vs[0], vs[1]);
    let mut mat: Vec<Vec<u64>> = readvv(n);
    mat.sort_by_key(|ab| ab[0]);
    let mut monay = 0;
    let mut k = 0;
    for ab in mat {
        if k + ab[1] > m as u64 {
            monay += ab[0] * ((m as u64) - k);
            break;
        }
        k += ab[1];
        monay += ab[0] * ab[1]
    }
    println!("{}", monay);
}
