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
    let mut path = Vec::new();
    path.push((sx, sy));
    let mut heap = BinaryHeap::new();
    let mut visited = BTreeSet::new();
    heap.push((path.clone(), sx, sy));
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some(p) = heap.pop() {
        if p.1 == ex && p.2 == ey {
            break;
        }
        visited.insert((p.1, p.2));
        for &d in dirs.iter() {
            let x = p.1 + d.0;
            let y = p.2 + d.1;
            if vv[y as usize][x as usize] == '#' {
                continue;
            }
            if visited.contains(&(x, y)) {
                continue;
            }
            let new_path = path.clone();
            path.push((x, y));
            heap.push((new_path, x, y));
        }
    }
    //println!("{path:?}");
    let mut cheats2: BTreeMap<i32, usize> = BTreeMap::new();
    let mut cheats20: BTreeMap<i32, usize> = BTreeMap::new();
    for i in 0..path.len() {
        for j in i + 1..path.len() {
            let p1 = path[i];
            let p2 = path[j];
            let d1 = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
            let d2: i32 = j as i32 - i as i32;
            if d1 <= 2 && d2 - d1 >= 100 {
                *cheats2.entry(d2 - d1).or_default() += 1;
            }
            if d1 <= 20 && d2 - d1 >= 100 {
                *cheats20.entry(d2 - d1).or_default() += 1;
            }
        }
    }
    //dbg!(&cheats);
    println!("{}", cheats2.values().sum::<usize>());
    println!("{}", cheats20.values().sum::<usize>());
}
