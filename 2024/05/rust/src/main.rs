use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut result1: i64 = 0;
    let mut result2: i64 = 0;
    let (input_rules, update) = input.split_once("\n\n").unwrap();
    let rules: Vec<(i64, i64)> = input_rules
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|p| (p.0.parse::<i64>().unwrap(), p.1.parse::<i64>().unwrap()))
        .collect();
    for line in update.lines() {
        let mut tokens: Vec<i64> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        let m: HashMap<i64, usize> = tokens
            .iter()
            .enumerate()
            .map(|e| return (e.1.clone(), e.0))
            .collect();
        let mut good = true;
        for &n in tokens.iter() {
            if !is_good(n, &m, &rules) {
                good = false;
                break;
            }
        }
        if good {
            result1 += tokens[tokens.len() / 2];
        } else {
            loop {
                let mut good = true;
                for i in 0..tokens.len() - 1 {
                    for j in i + 1..tokens.len() {
                        let a = tokens[i];
                        let b = tokens[j];
                        for rule in rules.iter() {
                            if a == rule.1 && b == rule.0 {
                                let t = a;
                                tokens[i] = b;
                                tokens[j] = t;
                                good = false;
                                break;
                            }
                        }
                    }
                }
                if good {
                    result2 += tokens[tokens.len() / 2];
                    break;
                }
            }
        }
    }
    println!("{result1}");
    println!("{result2}");
}

fn is_good(n: i64, m: &HashMap<i64, usize>, rules: &Vec<(i64, i64)>) -> bool {
    for rule in rules {
        if n == rule.0 && m.contains_key(&rule.1) && m.get(&rule.0) > m.get(&rule.1) {
            return false;
        }
        if n == rule.1 && m.contains_key(&rule.0) && m.get(&rule.0) > m.get(&rule.1) {
            return false;
        }
    }
    true
}
