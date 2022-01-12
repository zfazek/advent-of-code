use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let lines = std::fs::read_to_string("../04.txt").unwrap();
    one(&lines);
    two(&lines);
}

fn one(lines: &String) {
    let mut count = 0;
    'loop1: for line in lines.lines() {
        let mut words = HashSet::new();
        for word in line.split_whitespace() {
            if words.contains(&word) {
                continue 'loop1;
            }
            words.insert(word);
        }
        count += 1;
    }
    println!("{}", count);
}

fn two(lines: &String) {
    let mut count = 0;
    'loop1: for line in lines.lines() {
        let mut words = HashSet::new();
        for word in line.split_whitespace() {
            let word = word.chars().sorted().collect::<String>();
            if words.contains(&word) {
                continue 'loop1;
            }
            words.insert(word);
        }
        count += 1;
    }
    println!("{}", count);
}
