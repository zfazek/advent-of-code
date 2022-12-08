fn main() {
    let file = std::fs::read_to_string("../08.txt").unwrap();
    let mut m: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        let l: Vec<char> = line.chars().collect();
        m.push(l);
    }
    let n = m.len();
    let mut ans1 = 0;
    let mut ans2 = 0;
    for y in 0..n {
        for x in 0..n {
            if foo1(&m, n, x, y) {
                ans1 += 1;
            }
            let v = foo2(&m, n, x, y);
            if v > ans2 {
                ans2 = v;
            }
        }
    }
    println!("{}", ans1);
    println!("{}", ans2);
}

fn foo1(m: &Vec<Vec<char>>, n: usize, x: usize, y: usize) -> bool {
    (0..x).all(|i| m[y][i] < m[y][x]) || (x+1..n).all(|i| m[y][i] < m[y][x]) ||
    (0..y).all(|i| m[i][x] < m[y][x]) || (y+1..n).all(|i| m[i][x] < m[y][x])
}

fn foo2(m: &Vec<Vec<char>>, n: usize, x: usize, y: usize) -> usize {
    let mut s1 = (0..x).rev().take_while(|&i| m[y][i] < m[y][x]).count();
    if s1 < x {
        s1 += 1;
    }
    let mut s2 = (x+1..n).take_while(|&i| m[y][i] < m[y][x]).count();
    if s2 + x + 1 < n {
        s2 += 1;
    }
    let mut s3 = (0..y).rev().take_while(|&i| m[i][x] < m[y][x]).count();
    if s3 < y {
        s3 += 1;
    }
    let mut s4 = (y+1..n).take_while(|&i| m[i][x] < m[y][x]).count();
    if s4 + y + 1 < n {
        s4 += 1;
    }
    return s1 * s2 * s3 * s4;
}
