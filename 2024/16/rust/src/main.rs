use std::collections::BTreeSet;
use std::collections::BinaryHeap;

#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let input = include_str!("../../input.txt");
    let mut vv = Vec::new();
    for line in input.lines() {
        let v = line.chars().collect::<Vec<_>>();
        vv.push(v);
    }
    let mut end: Point = Default::default();
    let mut start: Point = Default::default();
    let width = vv[0].len() as i32;
    let height = vv.len() as i32;
    for i in 0..vv.len() {
        for j in 0..vv[i].len() {
            let c = vv[i][j];
            if c == 'S' {
                start.x = j as i32;
                start.y = i as i32;
            } else if c == 'E' {
                end.x = j as i32;
                end.y = i as i32;
            }
        }
    }
    let mut visited: BTreeSet<(i32, i32, i32, i32)> = BTreeSet::new();
    let mut heap: BinaryHeap<(i32, i32, i32, i32, i32)> = BinaryHeap::new();
    heap.push((0, start.x, start.y, 1, 0));
    while let Some(p) = heap.pop() {
        if !(0..width).contains(&p.1) || !(0..height).contains(&p.2) {
            continue;
        }
        if vv[p.2 as usize][p.1 as usize] == '#' {
            continue;
        }
        if visited.contains(&(p.1, p.2, p.3.abs(), p.4.abs())) {
            continue;
        }
        if p.1 == end.x && p.2 == end.y {
            println!("BINGO {}", -p.0);
            break;
        }
        heap.push((p.0 - 1, p.1 + p.3, p.2 + p.4, p.3, p.4));
        visited.insert((p.1, p.2, p.3, p.4));
        let d = turn_cw(p.3, p.4);
        heap.push((p.0 - 1001, p.1 + d.0, p.2 + d.1, d.0, d.1));
        let d = turn_ccw(p.3, p.4);
        heap.push((p.0 - 1001, p.1 + d.0, p.2 + d.1, d.0, d.1));
    }
}

fn turn_cw(dx: i32, dy: i32) -> (i32, i32) {
    if dx == 1 && dy == 0 {
        (0, 1)
    } else if dx == -1 && dy == 0 {
        (0, -1)
    } else if dx == 0 && dy == 1 {
        (-1, 0)
    } else {
        (1, 0)
    }
}

fn turn_ccw(dx: i32, dy: i32) -> (i32, i32) {
    if dx == 1 && dy == 0 {
        (0, -1)
    } else if dx == -1 && dy == 0 {
        (0, 1)
    } else if dx == 0 && dy == 1 {
        (1, 0)
    } else {
        (-1, 0)
    }
}
