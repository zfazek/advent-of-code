use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    let rules: HashSet<(i64, i64)> = rules_str
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let (mut result1, mut result2) = (0, 0);
    for line in updates_str.lines() {
        let pages: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        let mid = pages[pages.len() / 2];

        if is_valid(&pages, &rules) {
            result1 += mid;
        } else {
            result2 += sort_pages(&pages, &rules)[pages.len() / 2];
        }
    }
    println!("{result1}\n{result2}");
}

fn is_valid(pages: &[i64], rules: &HashSet<(i64, i64)>) -> bool {
    for i in 0..pages.len() {
        for j in i + 1..pages.len() {
            if rules.contains(&(pages[j], pages[i])) {
                return false;
            }
        }
    }
    true
}

fn sort_pages(pages: &[i64], rules: &HashSet<(i64, i64)>) -> Vec<i64> {
    let mut result = pages.to_vec();
    result.sort_by(|&a, &b| {
        if rules.contains(&(a, b)) {
            std::cmp::Ordering::Less
        } else if rules.contains(&(b, a)) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    result
}
