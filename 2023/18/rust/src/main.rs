use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../../inputa.txt");
    foo1(input);
    foo2(input);
}

fn foo1(input: &str) {
    let mut ps: BTreeMap<i32, Vec<(i32, i32, char)>> = BTreeMap::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut prev_d = 'U';
    for line in input.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let d = tokens.next().unwrap().chars().next().unwrap();
        let n = tokens.next().unwrap().parse::<i32>().unwrap() - 1;
        foo(&mut y, &mut x, d, &mut prev_d, n, &mut ps);
    }
    calc(ps);
}

fn foo2(input: &str) {
    let mut ps: BTreeMap<i32, Vec<(i32, i32, char)>> = BTreeMap::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut prev_d = 'U';
    for line in input.lines() {
        let token = line.split_once("(#").unwrap().1;
        let color = token[..5].to_string();
        let dirs = ['R', 'D', 'L', 'U'];
        let dd = token
            .split_once(')')
            .unwrap()
            .0
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;
        let d = dirs[dd];
        let n = i32::from_str_radix(&color, 16).unwrap() - 1;
        foo(&mut y, &mut x, d, &mut prev_d, n, &mut ps);
    }
    calc(ps);
}

fn foo(
    y: &mut i32,
    x: &mut i32,
    d: char,
    prev_d: &mut char,
    n: i32,
    ps: &mut BTreeMap<i32, Vec<(i32, i32, char)>>,
) {
    let mut dx: i32 = 0;
    let mut dy: i32 = 0;
    if d == 'R' {
        dx = 1;
        dy = 0;
    } else if d == 'L' {
        dx = -1;
        dy = 0;
    } else if d == 'D' {
        dx = 0;
        dy = 1;
    } else if d == 'U' {
        dx = 0;
        dy = -1;
    }
    if *prev_d == 'U' {
        if d == 'R' {
            ps.entry(*y).or_default().push((*x, 1, 'F'));
        } else {
            ps.entry(*y).or_default().push((*x, 1, '7'));
        }
    } else if *prev_d == 'D' {
        if d == 'R' {
            ps.entry(*y).or_default().push((*x, 1, 'L'));
        } else {
            ps.entry(*y).or_default().push((*x, 1, 'J'));
        }
    }
    if *prev_d == 'R' {
        if d == 'U' {
            ps.entry(*y).or_default().push((*x, 1, 'J'));
        } else {
            ps.entry(*y).or_default().push((*x, 1, '7'));
        }
    } else if *prev_d == 'L' {
        if d == 'D' {
            ps.entry(*y).or_default().push((*x, 1, 'F'));
        } else {
            ps.entry(*y).or_default().push((*x, 1, 'L'));
        }
    }
    *x += dx;
    *y += dy;
    if n > 0 {
        if d == 'R' {
            ps.entry(*y).or_default().push((*x, n, '-'));
            *x += dx * n;
        } else if d == 'L' {
            *x += dx * n;
            ps.entry(*y).or_default().push((*x + 1, n, '-'));
        } else {
            for _ in 0..n {
                ps.entry(*y).or_default().push((*x, 1, '|'));
                *y += dy;
            }
        }
    }
    *prev_d = d;
}

fn calc(mut ps: BTreeMap<i32, Vec<(i32, i32, char)>>) {
    let mut ans: i64 = 0;
    for row in ps.values_mut() {
        row.sort();
        let mut count = 0;
        let mut sum = 0;
        for i in 0..row.len() {
            sum += row[i].1;
            if ['|', 'L', 'J'].contains(&row[i].2) {
                count += 1;
            }
            if count % 2 == 1 && row[i].2 != '-' {
                sum += row[i + 1].0 - row[i].0 - 1;
            }
        }
        ans += sum as i64;
    }
    println!("{}", ans);
}
