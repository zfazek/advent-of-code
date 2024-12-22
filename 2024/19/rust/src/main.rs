use std::collections::BTreeSet;
use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("../../input.txt");
    let (towels_tokens, designs_tokens) = input.split_once("\n\n").unwrap();
    let towel_patterns: Vec<&str> = towels_tokens.split(", ").collect();
    let mut result1: usize = 0;
    let len = designs_tokens.lines().count();
    let vv: Vec<&str> = designs_tokens.lines().collect();
    'foo: for (n, &line) in vv.iter().enumerate() {
        let mut patterns_set = BTreeSet::new();
        for &p in towel_patterns.iter() {
            if line.contains(p) {
                patterns_set.insert(p);
            }
        }
        let mut heap = BinaryHeap::new();
        let mut visited = BTreeSet::new();
        heap.push((0, "".to_owned()));
        while let Some(p) = heap.pop() {
            visited.insert(p.1.to_owned());
            for &pattern in patterns_set.iter() {
                let word = p.1.to_owned() + pattern;
                //println!("{} {}", line, word);
                if word == line {
                    result1 += 1;
                    println!("{}/{}: {}", n + 1, len, result1);
                    continue 'foo;
                }
                if visited.contains(&word) {
                    continue;
                }
                if word.len() >= line.len() {
                    continue;
                }
                if !line.contains(&word) {
                    continue;
                }
                heap.push((word.len(), word));
            }
        }
        println!("{}/{}: {}", n + 1, len, result1);
    }
    println!("{}", result1);
}
