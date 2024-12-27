use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

fn main() {
    let input = include_str!("../../input.txt");
    let mut vv = Vec::new();
    for line in input.lines() {
        let v = line.chars().collect::<Vec<_>>();
        vv.push(v);
    }
    let mut sx: i32 = 0;
    let mut sy: i32 = 0;
    let mut ex: i32 = 0;
    let mut ey: i32 = 0;
    for i in 0..vv.len() {
        for j in 0..vv[i].len() {
            let c = vv[i][j];
            if c == 'S' {
                sx = j as i32;
                sy = i as i32;
            } else if c == 'E' {
                ex = j as i32;
                ey = i as i32;
            }
        }
    }
    let vv_orig = vv.clone();
    let n_moves = foo(&vv, sx, sy, ex, ey);
    let mut cheats: BTreeMap<(usize, usize), i32> = BTreeMap::new();
    for i in 1..vv.len() - 1 {
        for j in 1..vv[i].len() - 1 {
            vv = vv_orig.clone();
            let c = vv[i][j];
            if c == '#' {
                vv[i][j] = '.';
                let n = foo(&vv, sx, sy, ex, ey);
                if n < n_moves {
                    cheats.insert((j, i), n_moves - n);
                }
            }
        }
    }
    println!("{}", cheats.values().filter(|&x| *x >= 100).count());
}

fn foo(vv: &Vec<Vec<char>>, sx: i32, sy: i32, ex: i32, ey: i32) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut visited = BTreeSet::new();
    heap.push((0, sx, sy));
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some(p) = heap.pop() {
        if visited.contains(&(p.1, p.2)) {
            continue;
        }
        if p.1 == ex && p.2 == ey {
            return -p.0;
        }
        visited.insert((p.1, p.2));
        for &d in dirs.iter() {
            let x = p.1 + d.0;
            let y = p.2 + d.1;
            if vv[y as usize][x as usize] == '#' {
                continue;
            }
            heap.push((p.0 - 1, x, y));
        }
    }
    0
}
