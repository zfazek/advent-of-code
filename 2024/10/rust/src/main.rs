use std::collections::BTreeSet;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut result1 = BTreeSet::new();
    let mut result2 = BTreeSet::new();
    let vv = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let size = vv.len() as i32;
    for i in 0..size {
        for j in 0..size {
            if vv[i as usize][j as usize] != 0 {
                continue;
            }
            first(&vv, i, j, 0, size, (i, j), &mut result1);
        }
    }
    println!("{}", result1.len());
    for i in 0..size {
        for j in 0..size {
            if vv[i as usize][j as usize] != 0 {
                continue;
            }
            let mut trail = Vec::new();
            second(&vv, i, j, 0, size, &mut trail, &mut result2);
        }
    }
    println!("{}", result2.len());
}

fn first(
    vv: &Vec<Vec<i32>>,
    i: i32,
    j: i32,
    n: i32,
    size: i32,
    start: (i32, i32),
    result: &mut BTreeSet<(i32, i32, i32, i32)>,
) {
    if i < 0 || i >= size || j < 0 || j >= size {
        return;
    }
    let c = vv[i as usize][j as usize];
    if c != n {
        return;
    }
    if c == 9 {
        result.insert((start.0, start.1, i, j));
        return;
    }
    first(vv, i - 1, j, n + 1, size, start, result);
    first(vv, i + 1, j, n + 1, size, start, result);
    first(vv, i, j - 1, n + 1, size, start, result);
    first(vv, i, j + 1, n + 1, size, start, result);
}

fn second(
    vv: &Vec<Vec<i32>>,
    i: i32,
    j: i32,
    n: i32,
    size: i32,
    trail: &mut Vec<(i32, i32)>,
    result: &mut BTreeSet<Vec<(i32, i32)>>,
) {
    if i < 0 || i >= size || j < 0 || j >= size {
        return;
    }
    let c = vv[i as usize][j as usize];
    if c != n {
        return;
    }
    trail.push((i, j));
    if c == 9 {
        result.insert(trail.clone());
        return;
    }
    second(vv, i - 1, j, n + 1, size, trail, result);
    second(vv, i + 1, j, n + 1, size, trail, result);
    second(vv, i, j - 1, n + 1, size, trail, result);
    second(vv, i, j + 1, n + 1, size, trail, result);
    trail.pop();
}
