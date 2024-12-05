use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut result1: i64 = 0;
    let mut result2: i64 = 0;
    let (input_rules, update) = input.split_once("\n\n").unwrap();
    let mut rules = Vec::new();
    for rule in input_rules.lines() {
        let (a, b) = rule.split_once('|').unwrap();
        let a = a.parse::<i64>().unwrap();
        let b = b.parse::<i64>().unwrap();
        rules.push((a, b));
    }
    for line in update.lines() {
        let mut tokens = line
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let mut m = HashMap::new();
        for (i, &n) in tokens.iter().enumerate() {
            m.insert(n, i);
        }
        let mut good = true;
        for &n in tokens.iter() {
            if !is_good(n, &m, &rules) {
                good = false;
                break;
            }
        }
        if good {
            let v = tokens[tokens.len() / 2];
            result1 += v;
        } else {
            loop {
                let mut good = true;
                for i in 0..tokens.len() - 1 {
                    for j in i + 1..tokens.len() {
                        let a = tokens[i];
                        let b = tokens[j];
                        for &rule in &rules {
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
                    let v = tokens[tokens.len() / 2];
                    result2 += v;
                    break;
                }
            }
        }
    }
    println!("{result1}");
    println!("{result2}");
}

fn is_good(n: i64, m: &HashMap<i64, usize>, rules: &Vec<(i64, i64)>) -> bool {
    for j in 0..rules.len() {
        if n == rules[j].0 {
            if m.contains_key(&rules[j].1) && m.get(&rules[j].0) > m.get(&rules[j].1) {
                return false;
            }
        } else if n == rules[j].1
            && m.contains_key(&rules[j].0)
            && m.get(&rules[j].0) > m.get(&rules[j].1)
        {
            return false;
        }
    }
    true
}
