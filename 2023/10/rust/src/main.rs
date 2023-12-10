use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../../input.txt");
    let mut sx = 0;
    let mut sy = 0;
    let mut v = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let row = line.chars().collect::<Vec<_>>();
        v.push(row);
        if let Some(j) = line.find('S') {
            sx = j;
            sy = i;
        }
    }
    let mut ps = BTreeSet::new();
    let w = v[0].len();
    let h = v.len();
    let mut x = sx;
    let mut y = sy;
    println!("w, h: {} {}", w, h);
    println!("start pos: {} {}", sy, sx);
    let mut ans1 = 0;
    let mut dir = 0;
    while dir == 0 || x != sx || y != sy {
        let c = v[y][x];
        //println!("{} {}: dir: {}, c: {}", y, x, dir, c);
        if dir != 3 && y > 0 && (c == 'S' || c == '|' || c == 'L' || c == 'J') {
            let cc = v[y - 1][x];
            if cc == '|' || cc == '7' || cc == 'F' || cc == 'S' {
                ps.insert((x, y));
                y -= 1;
                dir = 1;
                ans1 += 1;
                continue;
            }
        }
        if dir != 1 && y < h - 1 && (c == 'S' || c == '|' || c == '7' || c == 'F') {
            let cc = v[y + 1][x];
            if cc == '|' || cc == 'L' || cc == 'J' || cc == 'S' {
                ps.insert((x, y));
                y += 1;
                dir = 3;
                ans1 += 1;
                continue;
            }
        }
        if dir != 4 && x > 0 && (c == 'S' || c == '-' || c == 'J' || c == '7') {
            let cc = v[y][x - 1];
            if cc == '-' || cc == 'L' || cc == 'F' || cc == 'S' {
                ps.insert((x, y));
                x -= 1;
                dir = 2;
                ans1 += 1;
                continue;
            }
        }
        if dir != 2 && x < w - 1 && (c == 'S' || c == '-' || c == 'L' || c == 'F') {
            let cc = v[y][x + 1];
            if cc == '-' || cc == 'J' || cc == '7' || cc == 'S' {
                ps.insert((x, y));
                x += 1;
                dir = 4;
                ans1 += 1;
                continue;
            }
        }
    }
    println!("{}", ans1 / 2);
    for y in 0..v.len() {
        for x in 0..v[y].len() {
            if !ps.contains(&(x, y)) {
                v[y][x] = '.';
            }
        }
    }
    'foo: for y in 0..v.len() {
        for x in 0..v[y].len() {
            if v[y][x] == 'S' {
                if v[y - 1][x] != '.' && v[y + 1][x] != '.' {
                    v[y][x] = '|';
                    break 'foo;
                }
                if v[y][x - 1] != '.' && v[y][x + 1] != '.' {
                    v[y][x] = '-';
                    break 'foo;
                }
                if v[y - 1][x] != '.' && v[y][x - 1] != '.' {
                    v[y][x] = 'J';
                    break 'foo;
                }
                if v[y - 1][x] != '.' && v[y][x + 1] != '.' {
                    v[y][x] = 'L';
                    break 'foo;
                }
                if v[y + 1][x] != '.' && v[y][x - 1] != '.' {
                    v[y][x] = '7';
                    break 'foo;
                }
                if v[y + 1][x] != '.' && v[y][x + 1] != '.' {
                    v[y][x] = 'F';
                    break 'foo;
                }
            }
        }
    }
    print(&v);
    let mut ans2 = 0;
    let tiles = BTreeSet::from_iter(vec!['S', '|', 'L', 'J']);
    for y in 0..v.len() {
        for x in 0..v[y].len() {
            if v[y][x] == '.' {
                let counter = v[y][0..x].iter().filter(|x| tiles.contains(&x)).count();
                if counter % 2 == 1 {
                    ans2 += 1;
                    //println!("{} {}", y, x);
                }
            }
        }
    }
    println!("{}", ans2);
}

fn print(v: &Vec<Vec<char>>) {
    for y in 0..v.len() {
        for x in 0..v[y].len() {
            print!("{}", v[y][x]);
        }
        println!();
    }
}
