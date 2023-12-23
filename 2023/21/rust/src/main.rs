use std::collections::BTreeSet;

struct Solution {
    sx: i32,
    sy: i32,
    vs: Vec<Vec<char>>,
    visited: BTreeSet<(i32, i32)>,
    ends: BTreeSet<(i32, i32)>,
}

impl Solution {
    fn new() -> Self {
        let mut sx = 0;
        let mut sy = 0;
        let mut vs = Vec::new();
        let mut visited = BTreeSet::new();
        let ends = BTreeSet::new();
        let input = include_str!("../../input.txt");
        for (y, line) in input.lines().enumerate() {
            let v = line.chars().collect::<Vec<_>>();
            if let Some(x) = line.find('S') {
                sx = x as i32;
                sy = y as i32;
            }
            vs.push(v);
        }
        vs[sy as usize][sx as usize] = '.';
        visited.insert((sy, sx));
        Solution {
            sx,
            sy,
            vs,
            visited,
            ends,
        }
    }
}
fn main() {
    let mut my = Solution::new();
    let h = my.vs.len() as i32;
    let w = my.vs[0].len() as i32;
    let n = 64;
    for i in 1..=n {
        for (y, x) in my.visited.clone() {
            if my.vs[y as usize][x as usize] != '#' {
                for step in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                    let xx = x + step.0;
                    let yy = y + step.1;
                    if xx >= 0 && xx < w && yy >= 0 && yy < h {
                        if my.vs[yy as usize][xx as usize] != '#' {
                            if i == n {
                                if (xx.abs_diff(my.sx) + yy.abs_diff(my.sy)) % 2 == 0 {
                                    my.ends.insert((yy + 1, xx + 1));
                                }
                            } else {
                                my.visited.insert((yy, xx));
                            }
                        }
                    }
                }
            }
        }
    }
    // println!("{:?}", my.ends);
    let ans = my.ends.len();
    println!("{ans}");
}
