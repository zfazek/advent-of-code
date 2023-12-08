use num::integer::lcm;
use std::collections::HashMap;

fn main() {
    foo1();
    foo2();
}

fn foo1() {
    let input = include_str!("../../input.txt");
    let instructions = input
        .split_once("\n\n")
        .unwrap()
        .0
        .chars()
        .collect::<Vec<_>>();
    let mut dp = HashMap::new();
    for line in input.split_once("\n\n").unwrap().1.lines() {
        let (k, v) = line.split_once(" = (").unwrap();
        let (v1, v2) = v.split_once(", ").unwrap();
        let v2 = v2.split_once(')').unwrap().0;
        dp.insert(k, (v1, v2));
    }
    let mut ans1: usize = 0;
    let mut pos = "AAA";
    let mut idx: usize = 0;
    while pos != "ZZZ" {
        if instructions[idx] == 'L' {
            pos = dp.get(&pos).unwrap().0;
        } else {
            pos = dp.get(&pos).unwrap().1;
        }
        idx += 1;
        idx %= instructions.len();
        ans1 += 1;
    }
    println!("{}", ans1);
}

fn foo2() {
    let input = include_str!("../../input.txt");
    let instructions = input
        .split_once("\n\n")
        .unwrap()
        .0
        .chars()
        .collect::<Vec<_>>();
    let mut dp = HashMap::new();
    for line in input.split_once("\n\n").unwrap().1.lines() {
        let (k, v) = line.split_once(" = (").unwrap();
        let (v1, v2) = v.split_once(", ").unwrap();
        let v2 = v2.split_once(')').unwrap().0;
        dp.insert(k, (v1, v2));
    }
    let mut positions = Vec::new();
    for e in dp.iter() {
        let k = e.0.chars().last().unwrap();
        if k == 'A' {
            positions.push(*e.0);
        }
    }
    let mut ans2: usize = 1;
    for p in positions {
        let mut counter: usize = 0;
        let mut idx: usize = 0;
        let mut pos = p;
        while !pos.ends_with('Z') {
            if instructions[idx] == 'L' {
                pos = dp.get(&pos).unwrap().0;
            } else {
                pos = dp.get(&pos).unwrap().1;
            }
            idx += 1;
            idx %= instructions.len();
            counter += 1;
        }
        ans2 = lcm(ans2, counter);
    }
    println!("{}", ans2);
}
