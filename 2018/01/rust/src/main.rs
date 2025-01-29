use std::collections::BTreeSet;

fn main() {
    let input = std::fs::read_to_string("../01.txt").unwrap();
    one(&input);
    two(&input);
}

fn one(input: &str) {
    let n = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .sum::<i32>();
    println!("{}", n);
}

fn two(input: &str) {
    let mut n = 0;
    let mut seen = BTreeSet::new();
    'l: loop {
        for line in input.lines() {
            n += line.parse::<i32>().unwrap();
            if seen.contains(&n) {
                println!("{}", n);
                break 'l;
            } else {
                seen.insert(n);
            }
        }
    }
}
