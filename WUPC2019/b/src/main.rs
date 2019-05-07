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
    let (h, w) = (vs[0], vs[1]);
    let mut mat: Vec<Vec<usize>> = readvv(h);

    if mat.iter().all(|l| l.iter().all(|&v| v == 0)) {
        println!("YES 0");
        return;
    }

    let zeroable = mat.iter().any(|l| l.iter().any(|&v| v == 5));
    if !zeroable {
        println!("NO");
        return;
    }

    let k;
    let maxvalue = *mat.iter().map(|l| l.iter().max().unwrap()).max().unwrap();
    if maxvalue > 5 {
        let mut results: Vec<usize> = Vec::new();
        for ii in 0..h {
            for jj in 0..w {
                if mat[ii][jj] != 5 {
                    continue;
                }
                let mut table: Vec<Vec<isize>> = vec![vec![0; w]; h];
                table[ii][jj] = -1;
                let mut p = 1;
                // 最後に5出ない数も含めて全部5にする
                let mut sum = 1;
                for i in 0..h {
                    for j in 0..w {
                        // if mat[i][j] == 5 {
                        //     continue;
                        // }
                        if table[i][j] != 0 {
                            continue;
                        }
                        let v = paint(&mut mat, &mut table, i, j, p);
                        // 領域の数じゃない。全てを1手で<5にできない
                        // 領域毎に、最大値が
                        // 0〜4: 0手
                        // 6, 7: 1手
                        // 8   : 2手
                        // 9   : 3手
                        if v <= 5 {
                            // sum += 0;
                        } else if v == 8 {
                            sum += 2;
                        } else if v == 9 {
                            sum += 3;
                        } else {
                            sum += 1;
                        }
                        p += 1;
                    }
                }
                results.push(sum);
            }
        }
        // 最小値をとる
        let minimum = results.iter().min().unwrap();
        k = *minimum;
    } else {
        k = 1;
    }
    println!("YES {}", k);
}

// max value をかえす
fn paint(mat: &mut Vec<Vec<usize>>, table: &mut Vec<Vec<isize>>, i: usize, j: usize, p: isize) -> usize {
    // if mat[i][j] == 5 {
    //     return 0;
    // }
    if table[i][j] != 0 {
        return 0;
    }
    table[i][j] = p;
    let mut tmp: Vec<usize> = vec![mat[i][j]; 5];
    // 4近傍もなんとかする
    if i > 1 {
        tmp[1] = paint(mat, table, i - 1, j,     p);
    }
    if i + 1 < mat.len() {
        tmp[2] = paint(mat, table, i + 1, j,     p);
    }
    if j > 1 {
        tmp[3] = paint(mat, table, i,     j - 1, p);
    }
    if j + 1 < mat[i].len() {
        tmp[4] = paint(mat, table, i,     j + 1, p);
    }
    let ret = tmp.iter().max().unwrap();
    *ret
}
