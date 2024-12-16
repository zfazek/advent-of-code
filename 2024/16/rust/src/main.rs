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
        let input = include_str!("../../input.txt");
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

    fn iter(&mut self, reindeer: &mut Reindeer, moves: &mut Vec<Move>) {
        if reindeer.point.x < 0
            || reindeer.point.x >= self.width
            || reindeer.point.y < 0
            || reindeer.point.y >= self.height
        {
            return;
        }
        if self.vv[reindeer.point.y as usize][reindeer.point.x as usize] == '#' {
            return;
        }
        if self.visited.contains(&(
            reindeer.point.x,
            reindeer.point.y,
            reindeer.dir.dx.abs(),
            reindeer.dir.dy.abs(),
        )) {
            return;
        }
        if reindeer.point.x == self.end.x && reindeer.point.y == self.end.y {
            self.paths.push(moves.clone());
            println!("{}", self.paths.len());
            let mut r: usize = 0;
            for m in moves.iter() {
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
            reindeer.point.x,
            reindeer.point.y,
            reindeer.dir.dx.abs(),
            reindeer.dir.dy.abs(),
        ));
        //print!("{:02} {:02} {:02} {:02} ", reindeer.point.y, reindeer.point.x, reindeer.dir.dy, reindeer.dir.dx);
        println!("visited: {:?}", self.visited.len());
        //println!("{:?}", moves);
        reindeer.point.x += reindeer.dir.dx;
        reindeer.point.y += reindeer.dir.dy;
        moves.push(Move::Move);
        self.iter(reindeer, moves);
        moves.pop();
        reindeer.point.x -= reindeer.dir.dx;
        reindeer.point.y -= reindeer.dir.dy;
        self.visited.remove(&(
            reindeer.point.x,
            reindeer.point.y,
            reindeer.dir.dx.abs(),
            reindeer.dir.dy.abs(),
        ));
        if self.visited.contains(&(
            reindeer.point.x,
            reindeer.point.y,
            reindeer.dir.dy.abs(),
            reindeer.dir.dx.abs(),
        )) {
            return;
        }
        reindeer.dir = turn_cw(&reindeer.dir);
        moves.push(Move::TurnCw);
        reindeer.point.x += reindeer.dir.dx;
        reindeer.point.y += reindeer.dir.dy;
        moves.push(Move::Move);
        self.iter(reindeer, moves);
        moves.pop();
        moves.pop();
        reindeer.point.x -= reindeer.dir.dx;
        reindeer.point.y -= reindeer.dir.dy;
        reindeer.dir = turn_ccw(&reindeer.dir);
        reindeer.dir = turn_ccw(&reindeer.dir);
        moves.push(Move::TurnCcw);
        reindeer.point.x += reindeer.dir.dx;
        reindeer.point.y += reindeer.dir.dy;
        moves.push(Move::Move);
        self.iter(reindeer, moves);
        moves.pop();
        moves.pop();
        reindeer.point.x -= reindeer.dir.dx;
        reindeer.point.y -= reindeer.dir.dy;
        reindeer.dir = turn_cw(&reindeer.dir);
    }
}

fn main() {
    let mut solution = Solution::new();
    solution.iter(&mut solution.reindeer.clone(), &mut solution.moves.clone());
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

fn turn_cw(d: &Dir) -> Dir {
    if d.dx == 1 && d.dy == 0 {
        return Dir { dx: 0, dy: 1 };
    } else if d.dx == -1 && d.dy == 0 {
        return Dir { dx: 0, dy: -1 };
    } else if d.dx == 0 && d.dy == 1 {
        return Dir { dx: -1, dy: 0 };
    } else {
        return Dir { dx: 1, dy: 0 };
    }
}

fn turn_ccw(d: &Dir) -> Dir {
    if d.dx == 1 && d.dy == 0 {
        return Dir { dx: 0, dy: -1 };
    } else if d.dx == -1 && d.dy == 0 {
        return Dir { dx: 0, dy: 1 };
    } else if d.dx == 0 && d.dy == 1 {
        return Dir { dx: 1, dy: 0 };
    } else {
        return Dir { dx: -1, dy: 0 };
    }
}
