use std::collections::BTreeSet;

struct Solution {
    sx: i32,
    sy: i32,
    tx: i32,
    ty: i32,
    vs: Vec<Vec<char>>,
    visited: BTreeSet<(i32, i32)>,
    path: BTreeSet<(i32, i32)>,
    max: usize,
}

impl Solution {
    fn new() -> Self {
        let max = 0;
        let mut sx = 0;
        let mut sy = 0;
        let mut tx = 0;
        let mut ty = 0;
        let mut vs = Vec::new();
        let visited = BTreeSet::new();
        let path = BTreeSet::new();
        let input = include_str!("../../input.txt");
        let h = input.lines().count();
        for (y, line) in input.lines().enumerate() {
            let v = line.chars().collect::<Vec<_>>();
            if y == 0 {
                if let Some(x) = line.find('.') {
                    sx = x as i32;
                    sy = y as i32;
                }
            } else if y == h - 1 {
                if let Some(x) = line.find('.') {
                    tx = x as i32;
                    ty = y as i32;
                }
            }
            vs.push(v);
        }
        Solution {
            sx,
            sy,
            tx,
            ty,
            vs,
            visited,
            path,
            max,
        }
    }
    fn foo(&mut self, x: i32, y: i32, part1: bool) {
        if x == self.tx && y == self.ty {
            if self.visited.len() > self.max {
                self.max = self.visited.len();
                self.path = self.visited.clone();
                println!("{}", self.max);
                if self.max == 6422 {
                    //std::process::exit(0);
                }
            }
            return;
        }
        let h = self.vs.len() as i32;
        let w = self.vs[0].len() as i32;
        for step in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let xx = x + step.1;
            let yy = y + step.0;
            if xx < 0 || xx >= w || yy < 0 || yy >= h {
                continue;
            }
            let c = self.vs[yy as usize][xx as usize];
            if c == '#' {
                continue;
            }
            if part1 && (c == '>' && step.1 == -1 || c == 'v' && step.0 == -1) {
                continue;
            }
            if self.visited.contains(&(yy, xx)) {
                continue;
            }
            self.visited.insert((yy, xx));
            self.foo(xx, yy, part1);
            self.visited.remove(&(yy, xx));
        }
    }

    fn _print(&self) {
        for y in 0..self.vs.len() {
            for x in 0..self.vs[y].len() {
                if self.path.contains(&(y as i32, x as i32)) {
                    print!("O");
                } else {
                    print!("{}", self.vs[y][x]);
                }
            }
            println!();
        }
        println!();
    }
}

fn main() {
    for part in [true, false] {
        let mut my = Solution::new();
        my.foo(my.sx, my.sy, part);
        let ans = my.max;
        // my._print();
        println!("answer: {ans}");
    }
}
