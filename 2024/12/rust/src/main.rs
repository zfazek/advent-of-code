use std::collections::BTreeSet;

#[derive(Debug)]
struct Shape {
    color: char,
    points: BTreeSet<(i32, i32)>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut vv = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let vv_orig = vv.clone();
    let len = vv.len() as i32;
    let mut shapes = Vec::new();
    for i in 0..len {
        for j in 0..len {
            let c = vv[i as usize][j as usize];
            if c == '.' {
                continue;
            }
            let points: BTreeSet<(i32, i32)> = BTreeSet::new();
            let mut shape = Shape { color: c, points };
            shape.points.insert((i, j));
            iter(&mut vv, i, j, &mut shape);
            shapes.push(shape);
        }
    }
    vv = vv_orig;
    let mut result1: usize = 0;
    let mut result2: usize = 0;
    for shape in shapes.iter() {
        let area = shape.points.len();
        let mut perimeters: BTreeSet<(i32, i32, i32, i32, Dir)> = BTreeSet::new();
        for &point in shape.points.iter() {
            let i = point.0;
            let j = point.1;
            if let Some(p) = get_perimeter(&vv, shape.color, i, j, i - 1, j) {
                perimeters.insert(p);
            }
            if let Some(p) = get_perimeter(&vv, shape.color, i, j, i + 1, j) {
                perimeters.insert(p);
            }
            if let Some(p) = get_perimeter(&vv, shape.color, i, j, i, j - 1) {
                perimeters.insert(p);
            }
            if let Some(p) = get_perimeter(&vv, shape.color, i, j, i, j + 1) {
                perimeters.insert(p);
            }
        }
        result1 += area * perimeters.len();
        let mut perimeters_new: BTreeSet<(i32, i32, i32, i32, Dir)> = BTreeSet::new();
        let mut perimeters_to_be_removed: BTreeSet<(i32, i32, i32, i32, Dir)> = BTreeSet::new();
        for p in perimeters.iter() {
            if perimeters_to_be_removed.contains(p) {
                continue;
            }
            let dx = p.3 - p.1;
            let dy = p.2 - p.0;
            let mut k = 0;
            loop {
                if !perimeters.contains(&(
                    p.0 + k * dy,
                    p.1 + k * dx,
                    p.0 + (k + 1) * dy,
                    p.1 + (k + 1) * dx,
                    p.4,
                )) {
                    break;
                }
                perimeters_to_be_removed.insert((
                    p.0 + k * dy,
                    p.1 + k * dx,
                    p.0 + (k + 1) * dy,
                    p.1 + (k + 1) * dx,
                    p.4,
                ));
                k += 1;
            }
            if k > 0 {
                perimeters_new.insert((p.0, p.1, p.0 + k * dy, p.1 + k * dx, p.4));
            } else {
                perimeters_new.insert(*p);
            }
        }
        result2 += area * perimeters_new.len();
    }
    println!("{}", result1);
    println!("{}", result2);
}

fn get_perimeter(
    vv: &Vec<Vec<char>>,
    color: char,
    i: i32,
    j: i32,
    y: i32,
    x: i32,
) -> Option<(i32, i32, i32, i32, Dir)> {
    let len = vv.len() as i32;
    if x < 0 {
        return Some((i, j, i + 1, j, Dir::Left));
    }
    if x == len {
        return Some((i, j + 1, i + 1, j + 1, Dir::Right));
    }
    if y < 0 {
        return Some((i, j, i, j + 1, Dir::Up));
    }
    if y == len {
        return Some((i + 1, j, i + 1, j + 1, Dir::Down));
    }
    let c = vv[y as usize][x as usize];
    if c != color {
        if x < j {
            return Some((i, j, i + 1, j, Dir::Left));
        }
        if x > j {
            return Some((i, j + 1, i + 1, j + 1, Dir::Right));
        }
        if y < i {
            return Some((i, j, i, j + 1, Dir::Up));
        }
        if y > i {
            return Some((i + 1, j, i + 1, j + 1, Dir::Down));
        }
    }
    None
}

fn iter(vv: &mut Vec<Vec<char>>, i: i32, j: i32, shape: &mut Shape) {
    let len = vv.len() as i32;
    if i < 0 || i == len || j < 0 || j == len {
        return;
    }
    let c = vv[i as usize][j as usize];
    if c != shape.color {
        return;
    }
    shape.points.insert((i, j));
    vv[i as usize][j as usize] = '.';
    iter(vv, i - 1, j, shape);
    iter(vv, i + 1, j, shape);
    iter(vv, i, j - 1, shape);
    iter(vv, i, j + 1, shape);
}
