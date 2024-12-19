use std::collections::BTreeMap;
use std::collections::BinaryHeap;

const W: i32 = 71;
const N: usize = 1024;

fn main() {
    let input = include_str!("../../input.txt");
    let mut vv = Vec::new();
    for _ in 0..W {
        vv.push(vec!['.'; W as usize]);
    }
    first(input, &vv);
    second(input, &vv);
}

fn first(input: &str, vv: &Vec<Vec<char>>) {
    let mut vv = vv.to_owned();
    let mut n = 0;
    for line in input.lines() {
        n += 1;
        let (x, y) = line.split_once(",").unwrap();
        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();
        vv[y][x] = '#';
        if n >= N {
            break;
        }
    }
    let tx = W - 1;
    let ty = W - 1;
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut visited = BTreeMap::new();
    _print(&vv, W, &visited);
    let mut heap = BinaryHeap::new();
    heap.push((0, 0, 0));
    while let Some(p) = heap.pop() {
        if visited.contains_key(&(p.1, p.2)) {
            continue;
        }
        visited.insert((p.1, p.2), p.0);
        for dir in dirs.iter() {
            let x = p.1 + dir.0;
            let y = p.2 + dir.1;
            if !(0..W).contains(&x) || !(0..W).contains(&y) {
                continue;
            }
            if vv[y as usize][x as usize] == '#' {
                continue;
            }
            heap.push((p.0 - 1, x, y));
        }
    }
    println!("{}", -visited.get(&(tx, ty)).unwrap());
}

fn second(input: &str, vv: &Vec<Vec<char>>) {
    let mut vv = vv.to_owned();
    for line in input.lines() {
        let (x, y) = line.split_once(",").unwrap();
        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();
        vv[y][x] = '#';
        let tx = W - 1;
        let ty = W - 1;
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut visited = BTreeMap::new();
        let mut heap = BinaryHeap::new();
        heap.push((0, 0, 0));
        while let Some(p) = heap.pop() {
            if visited.contains_key(&(p.1, p.2)) {
                continue;
            }
            visited.insert((p.1, p.2), p.0);
            for dir in dirs.iter() {
                let x = p.1 + dir.0;
                let y = p.2 + dir.1;
                if !(0..W).contains(&x) || !(0..W).contains(&y) {
                    continue;
                }
                if vv[y as usize][x as usize] == '#' {
                    continue;
                }
                heap.push((p.0 - 1, x, y));
            }
        }
        if !visited.contains_key(&(tx, ty)) {
            println!("{}", line);
            break;
        }
    }
}

fn _print(vv: &Vec<Vec<char>>, n: i32, visited: &BTreeMap<(i32, i32), i32>) {
    let n = n as usize;
    for i in 0..n {
        for j in 0..n {
            if visited.contains_key(&(j as i32, i as i32)) {
                print!("O");
            } else {
                print!("{}", vv[i][j]);
            }
        }
        println!();
    }
    println!();
}
