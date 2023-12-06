use std::collections::BTreeMap;
use std::io::{stdout, Write};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", foo1(input));
    println!("{}", foo2(input));
}

fn foo1(input: &str) -> i64 {
    let mut seeds: Vec<i64> = Vec::new();
    let mut rules: BTreeMap<(i64, i64), i64> = BTreeMap::new();
    let mut vec = Vec::new();
    for line in input.lines() {
        if line.starts_with("seeds:") {
            seeds = line[7..]
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
        }
        if !line.is_empty()
            && !line.starts_with(' ')
            && !line.chars().next().unwrap().is_ascii_digit()
        {
            rules.clear();
        }
        if !line.is_empty() && line.chars().next().unwrap().is_ascii_digit() {
            let row = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let from = row[1];
            let diff = row[0] - from;
            let range = row[2];
            rules.insert((from, range), diff);
        }
        if line.is_empty() && !rules.is_empty() {
            vec.push(rules.clone());
        }
    }
    let mut min = i64::MAX;
    for i in 0..seeds.len() {
        let mut seed = seeds[i];
        for v in vec.iter() {
            for rule in v.iter() {
                let from = rule.0 .0;
                let range = rule.0 .1;
                let diff = rule.1;
                if seed >= from && seed < from + range {
                    seed += diff;
                    break;
                }
            }
        }
        if seed < min {
            min = seed;
        }
    }
    min
}

fn foo2(input: &str) -> i64 {
    let mut seeds: Vec<i64> = Vec::new();
    let mut rules: BTreeMap<(i64, i64), i64> = BTreeMap::new();
    let mut vec = Vec::new();
    for line in input.lines() {
        if line.starts_with("seeds:") {
            seeds = line[7..]
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
        }
        if !line.is_empty()
            && !line.starts_with(' ')
            && !line.chars().next().unwrap().is_ascii_digit()
        {
            rules.clear();
        }
        if !line.is_empty() && line.chars().next().unwrap().is_ascii_digit() {
            let row = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let from = row[1];
            let diff = row[0] - from;
            let range = row[2];
            rules.insert((from, range), diff);
        }
        if line.is_empty() && !rules.is_empty() {
            vec.push(rules.clone());
        }
    }
    let mut n: i64 = 0;
    for i in (0..seeds.len()).step_by(2) {
        n += seeds[i + 1];
    }
    let mut counter: i64 = 0;
    let mut min = i64::MAX;
    for i in (0..seeds.len()).step_by(2) {
        for s in seeds[i]..seeds[i] + seeds[i + 1] {
            let mut seed = s;
            counter += 1;
            if counter % 100000 == 0 {
                print!("\r{:.1}%", counter as f32 / n as f32 * 100.0);
                stdout().flush().unwrap();
            }
            for v in vec.iter() {
                for rule in v.iter() {
                    let from = rule.0 .0;
                    let range = rule.0 .1;
                    let diff = rule.1;
                    if seed >= from && seed < from + range {
                        seed += diff;
                        break;
                    }
                }
            }
            if seed < min {
                min = seed;
            }
        }
    }
    println!();
    min
}
