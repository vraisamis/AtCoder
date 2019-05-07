fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim();
    let ss: Vec<_> = s.chars().collect();
    let mut cnt = 0;
    for c in ss {
        if c == 'W' {
            cnt += 1;
        } else if c == 'A' && cnt > 0 {
            print!("A");
            for _ in 0..cnt {print!("C")}
            cnt = 0;
        } else {
            if cnt > 0 {
                for _ in 0..cnt {print!("W")}
                cnt = 0;
            }
            print!("{}", c);
        }
    }
    println!("");
    // while s.contains("WA") {
    //     s = s.replace("WA", "AC");
    // }
    // println!("{}", s);
}
