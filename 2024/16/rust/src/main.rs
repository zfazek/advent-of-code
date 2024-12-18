use std::collections::BTreeSet;

#[derive(Default, Debug, Clone, Copy)]
struct Reindeer {
    point: Point,
    dir: Dir,
}

#[derive(Default, Debug, Clone, Copy)]
struct Dir {
    dy: i32,
    dx: i32,
}

#[derive(Default, Debug, Clone, Copy)]
struct Point {
    y: i32,
    x: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Move {
    Move,
    TurnCw,
    TurnCcw,
}

struct Solution {
    vv: Vec<Vec<char>>,
    end: Point,
    reindeer: Reindeer,
    width: i32,
    height: i32,
    moves: Vec<Move>,
    paths: Vec<Vec<Move>>,
    visited: BTreeSet<(i32, i32, i32, i32)>,
}

impl Solution {
    fn new() -> Self {
        let input = include_str!("../../inputa.txt");
        let mut vv = Vec::new();
        for line in input.lines() {
            let v = line.chars().collect::<Vec<_>>();
            vv.push(v);
        }
        let mut end: Point = Default::default();
        let mut start: Point = Default::default();
        let mut reindeer: Reindeer = Default::default();
        let width = vv[0].len() as i32;
        let height = vv.len() as i32;
        for i in 0..vv.len() {
            for j in 0..vv[i].len() {
                let c = vv[i][j];
                if c == 'S' {
                    start.x = j as i32;
                    start.y = i as i32;
                    reindeer.point.x = j as i32;
                    reindeer.point.y = i as i32;
                    reindeer.dir.dx = 1 as i32;
                    reindeer.dir.dy = 0 as i32;
                } else if c == 'E' {
                    end.x = j as i32;
                    end.y = i as i32;
                }
            }
        }
        let moves: Vec<Move> = Vec::new();
        let paths: Vec<Vec<Move>> = Vec::new();
        let visited: BTreeSet<(i32, i32, i32, i32)> = BTreeSet::new();
        Solution {
            vv,
            end,
            reindeer,
            width,
            height,
            moves,
            paths,
            visited,
        }
    }

    fn foo(&mut self) {
        if self.reindeer.point.x < 0
            || self.reindeer.point.x >= self.width
            || self.reindeer.point.y < 0
            || self.reindeer.point.y >= self.height
        {
            return;
        }
        if self.vv[self.reindeer.point.y as usize][self.reindeer.point.x as usize] == '#' {
            return;
        }
        if self.visited.contains(&(
            self.reindeer.point.x,
            self.reindeer.point.y,
            self.reindeer.dir.dx.abs(),
            self.reindeer.dir.dy.abs(),
        )) {
            return;
        }
        if self.reindeer.point.x == self.end.x && self.reindeer.point.y == self.end.y {
            self.paths.push(self.moves.clone());
            println!("{}", self.moves.len());
            let mut r: usize = 0;
            for m in self.moves.iter() {
                if *m == Move::Move {
                    r += 1;
                } else {
                    r += 1000;
                }
            }
            println!("{}", r);
            return;
        }
        self.visited.insert((
            self.reindeer.point.x,
            self.reindeer.point.y,
            self.reindeer.dir.dx.abs(),
            self.reindeer.dir.dy.abs(),
        ));
        //print!("{:02} {:02} {:02} {:02} ", self.reindeer.point.y, self.reindeer.point.x, self.reindeer.dir.dy, self.reindeer.dir.dx);
        //println!("visited: {:?}", self.visited.len());
        //println!("{:?}", moves);
        self.mov();
        self.foo();
        self.mov_back();
        self.turn_cw();
        self.moves.push(Move::TurnCw);
        self.mov();
        self.foo();
        self.mov_back();
        self.moves.pop();
        self.turn_back();
        self.moves.push(Move::TurnCcw);
        self.mov();
        self.foo();
        self.mov_back();
        self.moves.pop();
        self.turn_cw();
    }

    fn mov(&mut self) {
        self.reindeer.point.x += self.reindeer.dir.dx;
        self.reindeer.point.y += self.reindeer.dir.dy;
        self.moves.push(Move::Move);
    }

    fn mov_back(&mut self) {
        self.reindeer.point.x -= self.reindeer.dir.dx;
        self.reindeer.point.y -= self.reindeer.dir.dy;
        self.moves.pop();
        self.visited.remove(&(
            self.reindeer.point.x,
            self.reindeer.point.y,
            self.reindeer.dir.dx.abs(),
            self.reindeer.dir.dy.abs(),
        ));
    }

    fn turn_cw(&mut self) {
        if self.reindeer.dir.dx == 1 && self.reindeer.dir.dy == 0 {
            self.reindeer.dir.dx = 0;
            self.reindeer.dir.dy = 1;
        } else if self.reindeer.dir.dx == -1 && self.reindeer.dir.dy == 0 {
            self.reindeer.dir.dx = 0;
            self.reindeer.dir.dy = -1;
        } else if self.reindeer.dir.dx == 0 && self.reindeer.dir.dy == 1 {
            self.reindeer.dir.dx = -1;
            self.reindeer.dir.dy = 0;
        } else {
            self.reindeer.dir.dx = 1;
            self.reindeer.dir.dy = 0;
        }
    }

    #[allow(dead_code)]
    fn turn_ccw(&mut self) {
        if self.reindeer.dir.dx == 1 && self.reindeer.dir.dy == 0 {
            self.reindeer.dir.dx = 0;
            self.reindeer.dir.dy = -1;
        } else if self.reindeer.dir.dx == -1 && self.reindeer.dir.dy == 0 {
            self.reindeer.dir.dx = 0;
            self.reindeer.dir.dy = 1;
        } else if self.reindeer.dir.dx == 0 && self.reindeer.dir.dy == 1 {
            self.reindeer.dir.dx = 1;
            self.reindeer.dir.dy = 0;
        } else {
            self.reindeer.dir.dx = -1;
            self.reindeer.dir.dy = 0;
        }
    }

    fn turn_back(&mut self) {
        self.reindeer.dir.dx *= -1;
        self.reindeer.dir.dy *= -1;
    }
}

fn main() {
    let mut solution = Solution::new();
    solution.foo();
    //println!("ZZZ {:?}", solution.paths);
    let mut result1 = std::usize::MAX;
    for p in solution.paths.iter() {
        let mut r: usize = 0;
        for m in p.iter() {
            if *m == Move::Move {
                r += 1;
            } else {
                r += 1000;
            }
        }
        if r < result1 {
            result1 = r;
        }
    }
    println!("{}", result1);
}
