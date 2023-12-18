use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;

#[derive(Clone, Debug, Eq)]
struct Point {
    cost: i32,
    y: i32,
    x: i32,
    dir: i32,
    in_dir: i32,
}

impl Point {
    fn new_with(cost: i32, y: i32, x: i32, dir: i32, in_dir: i32) -> Self {
        Point {
            cost,
            y,
            x,
            dir,
            in_dir,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let mut vs = Vec::new();
    for line in input.lines() {
        let v = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>();
        vs.push(v);
    }
    let w = vs[0].len() as i32;
    let h = vs.len() as i32;
    let target_x = w - 1;
    let target_y = h - 1;
    vs[0][0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Point::new_with(0, 0, 0, -1, -1));
    let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut visited = BTreeMap::new();
    //_print(&vs, w, h);
    while let Some(p) = heap.pop() {
        if visited.contains_key(&(p.y, p.x, p.dir, p.in_dir)) {
            continue;
        }
        visited.insert((p.y, p.x, p.dir, p.in_dir), p.cost);
        for (new_dir, &dir_) in dirs.iter().enumerate() {
            let new_dir = new_dir as i32;
            let x = p.x + dir_.1;
            let y = p.y + dir_.0;
            let mut new_indir = 1;
            if p.dir == new_dir {
                new_indir = p.in_dir + 1;
            }
            let not_reverse = (new_dir + 2) % dirs.len() as i32 != p.dir;
            //let valid = new_indir <= 3;
            let valid = new_indir <= 10 && (new_dir == p.dir || p.in_dir >= 4 || p.in_dir == -1);
            if x >= 0 && x < w && y >= 0 && y < h && not_reverse && valid {
                if visited.contains_key(&(y, x, new_dir, new_indir)) {
                    continue;
                }
                let cost = vs[y as usize][x as usize];
                let next = Point::new_with(p.cost + cost, y, x, new_dir, new_indir);
                heap.push(next);
            }
        }
    }
    let mut ans1: i32 = 1000000000;
    for (k, &v) in visited.iter() {
        if k.1 == target_x && k.0 == target_y && v < ans1 {
            ans1 = v;
        }
    }
    println!("{}", ans1);
}

fn _print(vs: &Vec<Vec<i32>>, w: i32, h: i32) {
    for i in 0..h as usize {
        for j in 0..w as usize {
            print!("{:2} ", vs[i][j]);
        }
        println!();
    }
    println!();
}
