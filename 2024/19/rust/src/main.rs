use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../../inputa.txt");
    let (towels_tokens, designs_tokens) = input.split_once("\n\n").unwrap();
    let towel_patterns: Vec<&str> = towels_tokens.split(", ").collect();
    let mut result1: usize = 0;
    let len = designs_tokens.lines().count();
    let vv: Vec<&str> = designs_tokens.lines().collect();
    vv.iter().enumerate().for_each(|(n, &line)| {
        let mut patterns_set = BTreeSet::new();
        for &p in towel_patterns.iter() {
            if line.contains(p) {
                patterns_set.insert(p);
            }
        }
        let max_len = patterns_set.iter().map(|x| x.len()).max().unwrap();
        let res = foo(line, 0, &patterns_set, max_len);
        if res {
            result1 += 1;
        }
        println!("{}/{}: {}", n + 1, len, result1);
    });
    println!("{}", result1);
}

fn foo(design: &str, i: usize, towel_patterns: &BTreeSet<&str>, max_len: usize) -> bool {
    let len = design.len();
    for j in 1..len {
        if i + j > len {
            return false;
        }
        if j > max_len {
            return false;
        }
        let p = &design[i..i + j];
        if towel_patterns.contains(p) {
            if i + j == design.len() {
                return true;
            }
            let res = foo(design, i + j, towel_patterns, max_len);
            if res {
                return true;
            }
        }
    }
    false
}
