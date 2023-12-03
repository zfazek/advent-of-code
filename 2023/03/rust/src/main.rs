use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let ans = foo(&input);
    println!("{}", ans.0);
    println!("{}", ans.1);
}

fn foo(input: &str) -> (usize, usize) {
    let mut ans1: usize = 0;
    let mut ans2: usize = 0;
    let mut map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut chars: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut l: Vec<char> = line.chars().collect();
        l.push('.');
        chars.push(l);
    }
    for i in 0..chars.len() {
        let mut n = 0;
        let mut adjacent = false;
        let mut x: usize = 0;
        let mut y: usize = 0;
        for j in 0..chars[0].len() {
            if !chars[i][j].is_digit(10) {
                if n > 0 {
                    if adjacent {
                        ans1 += n as usize;
                        let _ = map.entry((x, y)).or_insert_with(Vec::new).push(n);
                        adjacent = false;
                    }
                    n = 0;
                }
            } else {
                n = n * 10 + chars[i][j].to_digit(10).unwrap();
                if !adjacent {
                    for di in 0..=2 {
                        for dj in 0..=2 {
                            if j + dj > 0 && i + di > 0 && i + di < chars.len() {
                                let _y = i + di - 1;
                                let _x = j + dj - 1;
                                if !chars[_y][_x].is_digit(10) && chars[_y][_x] != '.' {
                                    adjacent = true;
                                    y = _y;
                                    x = _x;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for e in map {
        if e.1.len() == 2 {
            ans2 += (e.1[0] * e.1[1]) as usize;
        }
    }
    (ans1, ans2)
}
