use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;

#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, Ord)]
struct Path {
    score: i32,
    last: Point,
    path: BTreeSet<Point>,
}

impl Path {
    fn new_with(score: i32, x: i32, y: i32, dx: i32, dy: i32) -> Self {
        let last = Point { x, y, dx, dy };
        let mut path = BTreeSet::new();
        path.insert(last.clone());
        Path { score, last, path }
    }

    fn add(&mut self, score: i32, x: i32, y: i32, dx: i32, dy: i32) {
        self.score -= score;
        let last = Point { x, y, dx, dy };
        self.last = last.clone();
        let point = Point {
            x,
            y,
            dx: dx.abs(),
            dy: dy.abs(),
        };
        self.path.insert(point);
    }

    #[allow(dead_code)]
    fn contains(&self, x: i32, y: i32, dx: i32, dy: i32) -> bool {
        let point = Point {
            x,
            y,
            dx: dx.abs(),
            dy: dy.abs(),
        };
        self.path.contains(&point)
    }
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
                start.dx = 1;
                start.dy = 0;
            } else if c == 'E' {
                end.x = j as i32;
                end.y = i as i32;
            }
        }
    }
    first(&vv, &end, &start, width, height);
    second(&vv, &end, &start, width, height);
}

fn first(vv: &Vec<Vec<char>>, end: &Point, start: &Point, width: i32, height: i32) {
    let mut visited: BTreeSet<(i32, i32, i32, i32)> = BTreeSet::new();
    let mut heap: BinaryHeap<(i32, i32, i32, i32, i32)> = BinaryHeap::new();
    heap.push((0, start.x, start.y, start.dx, start.dy));
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
        visited.insert((p.1, p.2, p.3.abs(), p.4.abs()));
        heap.push((p.0 - 1, p.1 + p.3, p.2 + p.4, p.3, p.4));
        let d = turn_cw(p.3, p.4);
        heap.push((p.0 - 1001, p.1 + d.0, p.2 + d.1, d.0, d.1));
        let d = turn_ccw(p.3, p.4);
        heap.push((p.0 - 1001, p.1 + d.0, p.2 + d.1, d.0, d.1));
    }
}

fn second(vv: &Vec<Vec<char>>, end: &Point, start: &Point, width: i32, height: i32) {
    let mut visited: BTreeMap<(i32, i32, i32, i32), usize> = BTreeMap::new();
    let mut paths = BTreeSet::new();
    let mut heap: BinaryHeap<Path> = BinaryHeap::new();
    let path = Path::new_with(0, start.x, start.y, 1, 0);
    heap.push(path);
    let mut max = i32::MIN;
    while let Some(path) = heap.pop() {
        let point = path.last.clone();
        if path.score < max {
            continue;
        }
        if !(0..width).contains(&point.x) || !(0..height).contains(&point.y) {
            continue;
        }
        if vv[point.y as usize][point.x as usize] == '#' {
            continue;
        }
        if let Some(&v) = visited.get(&(point.x, point.y, point.dx.abs(), point.dy.abs())) {
            if v > 100 {
                continue;
            }
        }
        //println!("{:?}", &path);
        if end.x == point.x && end.y == point.y {
            max = path.score;
            paths.insert(path.to_owned());
            println!("BINGO {} {}", max, path.path.len());
            continue;
        }
        let pp = point.to_owned();
        let entry = (pp.x, pp.y, pp.dx.abs(), pp.dy.abs());
        *visited.entry(entry).or_default() += 1;
        let mut new_point = point.clone();
        new_point.x += new_point.dx;
        new_point.y += new_point.dy;
        let mut new_path = path.clone();
        new_path.add(1, new_point.x, new_point.y, new_point.dx, new_point.dy);
        heap.push(new_path);

        let mut new_point = point.clone();
        let d = turn_cw(point.dx, point.dy);
        new_point.dx = d.0;
        new_point.dy = d.1;
        new_point.x += new_point.dx;
        new_point.y += new_point.dy;
        let mut new_path = path.clone();
        new_path.add(1001, new_point.x, new_point.y, new_point.dx, new_point.dy);
        heap.push(new_path.clone());

        let mut new_point = point.clone();
        let d = turn_ccw(point.dx, point.dy);
        new_point.dx = d.0;
        new_point.dy = d.1;
        new_point.x += new_point.dx;
        new_point.y += new_point.dy;
        let mut new_path = path.clone();
        new_path.add(1001, new_point.x, new_point.y, new_point.dx, new_point.dy);
        heap.push(new_path.clone());
    }
    let mut dp: BTreeMap<i32, usize> = BTreeMap::new();
    for path in paths.iter() {
        *dp.entry(path.score).or_default() += 1;
    }
    let &min = dp.last_key_value().unwrap().0;
    let mut points: BTreeSet<(i32, i32)> = BTreeSet::new();
    for path in paths.iter() {
        if path.score == min {
            for p in path.path.iter() {
                points.insert((p.x, p.y));
            }
        }
    }
    println!("{}", points.len());
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
