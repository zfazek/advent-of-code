use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("../../input.txt");
    let (towels_tokens, designs_tokens) = input.split_once("\n\n").unwrap();
    let towel_patterns: Vec<&str> = towels_tokens.split(", ").collect();
    let mut result1: usize = 0;
    let mut result2: usize = 0;
    let vv: Vec<&str> = designs_tokens.lines().collect();
    for &line in vv.iter() {
        let mut patterns_set = BTreeSet::new();
        for &p in towel_patterns.iter() {
            if line.contains(p) {
                patterns_set.insert(p);
            }
        }
        let mut heap = BinaryHeap::new();
        let mut visited: BTreeMap<String, usize> = BTreeMap::new();
        heap.push((0, "".to_owned()));
        'loop1: while let Some(p) = heap.pop() {
            *visited.entry(p.1.to_owned()).or_default() += 1;
            for &pattern in patterns_set.iter() {
                let word = p.1.to_owned() + pattern;
                if word == line {
                    result1 += 1;
                    let mut dp: BTreeMap<String, usize> = BTreeMap::new();
                    result2 += foo(line, "".to_owned(), &patterns_set, &mut dp);
                    break 'loop1;
                }
                if word.len() >= line.len() {
                    continue;
                }
                if !line.starts_with(&word) {
                    continue;
                }
                if visited.contains_key(&word) && *visited.get(&word).unwrap() > 0 {
                    continue;
                }
                heap.push((word.len(), word));
            }
        }
    }
    println!("{}", result1);
    println!("{}", result2);
}

fn foo(
    line: &str,
    word: String,
    patterns_set: &BTreeSet<&str>,
    dp: &mut BTreeMap<String, usize>,
) -> usize {
    if dp.contains_key(&word) {
        return *dp.get(&word).unwrap();
    }
    if word == line {
        return 1;
    }
    if word.len() >= line.len() {
        return 0;
    }
    if !line.starts_with(&word) {
        return 0;
    }
    let mut result: usize = 0;
    for &pattern in patterns_set.iter() {
        let word = word.to_owned() + pattern;
        result += foo(line, word.to_owned(), patterns_set, dp);
    }
    dp.insert(word, result);
    result
}
