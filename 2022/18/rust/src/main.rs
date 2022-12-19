fn main() {
    let file = std::fs::read_to_string("../18.txt").unwrap();
    let mut s = std::collections::HashSet::new();
    for line in file.lines() {
        let mut tokens = line.split(",");
        let x = tokens.next().unwrap().parse::<i32>().unwrap();
        let y = tokens.next().unwrap().parse::<i32>().unwrap();
        let z = tokens.next().unwrap().parse::<i32>().unwrap();
        s.insert((x, y, z));
    }
    let mut ans = 0;
    for (x, y, z) in &s {
        if !s.contains(&(*x + 1, *y, *z)) {
            ans += 1;
        }
        if !s.contains(&(*x - 1, *y, *z)) {
            ans += 1;
        }
        if !s.contains(&(*x, *y + 1, *z)) {
            ans += 1;
        }
        if !s.contains(&(*x, *y - 1, *z)) {
            ans += 1;
        }
        if !s.contains(&(*x, *y, *z + 1)) {
            ans += 1;
        }
        if !s.contains(&(*x, *y, *z - 1)) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
