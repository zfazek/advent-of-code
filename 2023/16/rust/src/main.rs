use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Point {
    fn new_with(x: i32, y: i32, dx: i32, dy: i32) -> Self {
        Point { x, y, dx, dy }
    }

    fn mov(&mut self, dx: i32, dy: i32) {
        self.dx = dx;
        self.dy = dy;
        self.x += self.dx;
        self.y += self.dy;
    }

    fn reset(&mut self) {
        self.dx = 0;
        self.dy = 0;
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let mut vs = Vec::new();
    for line in input.lines() {
        vs.push(line.chars().collect::<Vec<_>>());
    }
    let w = vs[0].len() as i32;
    let h = vs.len() as i32;
    let mut start_positions = Vec::new();
    start_positions.push(Point::new_with(0, 0, 1, 0));
    foo(&vs, &start_positions);
    start_positions.clear();
    for i in 0..h {
        for j in 0..w {
            if i > 0 && i < h - 1 && j > 0 && j < w - 1
                || i == 0 && j == 0
                || i == h - 1 && j == 0
                || i == 0 && j == w - 1
                || i == h - 1 && j == w - 1
            {
                continue;
            }
            if i == 0 {
                start_positions.push(Point::new_with(j, i, 0, 1));
            } else if i == h - 1 {
                start_positions.push(Point::new_with(j, i, 0, -1));
            } else if j == 0 {
                start_positions.push(Point::new_with(j, i, 1, 0));
            } else if j == w - 1 {
                start_positions.push(Point::new_with(j, i, -1, 0));
            }
        }
    }
    foo(&vs, &start_positions);
}

fn foo(v: &Vec<Vec<char>>, start_positions: &Vec<Point>) {
    let mut ans = 0;
    let w = v[0].len() as i32;
    let h = v.len() as i32;
    for start_pos in start_positions {
        let mut vs = v.clone();
        let mut beams = Vec::new();
        beams.push(start_pos.clone());
        let mut history = HashSet::new();
        let mut cells = HashSet::new();
        while !beams.is_empty() {
            let mut new_beams = Vec::new();
            for beam in beams.iter_mut() {
                let c = vs[beam.y as usize][beam.x as usize];
                cells.insert((beam.y, beam.x));
                match c {
                    '.' | '#' => {
                        vs[beam.y as usize][beam.x as usize] = '#';
                        beam.mov(beam.dx, beam.dy);
                    }
                    '-' => {
                        if beam.dy == 0 {
                            beam.mov(beam.dx, beam.dy);
                        } else {
                            let new_beam = Point::new_with(beam.x - 1, beam.y, -1, 0);
                            new_beams.push(new_beam);
                            beam.mov(1, 0);
                        }
                    }
                    '|' => {
                        if beam.dx == 0 {
                            beam.mov(beam.dx, beam.dy);
                        } else {
                            let new_beam = Point::new_with(beam.x, beam.y - 1, 0, -1);
                            new_beams.push(new_beam);
                            beam.mov(0, 1);
                        }
                    }
                    '/' => {
                        beam.mov(-beam.dy, -beam.dx);
                    }
                    '\\' => {
                        beam.mov(beam.dy, beam.dx);
                    }
                    _ => {}
                }
                if history.contains(&*beam) {
                    beam.reset();
                } else {
                    history.insert(beam.clone());
                }
            }
            beams.append(&mut new_beams);
            beams.retain(|beam| {
                !(beam.dx == 0 && beam.dy == 0)
                    && beam.x >= 0
                    && beam.x < w
                    && beam.y >= 0
                    && beam.y < h
            });
        }
        //_print(&vs);
        let res = cells.len();
        if res > ans {
            ans = res;
        }
    }
    println!("{}", ans);
}

fn _print(vs: &Vec<Vec<char>>) {
    for line in vs.iter() {
        for c in line.iter() {
            print!("{}", c);
        }
        println!();
    }
    println!();
}
