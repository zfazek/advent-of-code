use std::collections::BTreeSet;

const V: usize = 999999;

fn main() {
    let input = include_str!("../../input.txt");
    let mut dp = BTreeSet::new();
    let mut y = 0;
    let mut cols = BTreeSet::new();
    for line in input.lines() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                cols.insert(x);
                dp.insert((y, x));
            }
        }
        if !line.contains('#') {
            y += V;
        }
        y += 1;
    }
    //println!("{:?}", dp);
    let mut empty_cols = BTreeSet::new();
    for x in 0..*cols.iter().max().unwrap() {
        if !cols.contains(&x) {
            empty_cols.insert(x);

        }
    }
    let mut dp1 = BTreeSet::new();
    for &(y, x) in dp.iter() {
       let mut dx = 0;
       for &c in empty_cols.iter() {
            if c < x {
                dx += V;
            }
        }
       dp1.insert((y, x + dx));
    }
    dp = dp1;
    //println!("{:?}", dp);
    let mut ans1: usize = 0;
    for &e1 in dp.iter() {
        for &e2 in dp.iter() {
            ans1 += e1.0.abs_diff(e2.0) + e1.1.abs_diff(e2.1);
        }
    }
    ans1 /= 2;
    println!("{}", ans1);
}
