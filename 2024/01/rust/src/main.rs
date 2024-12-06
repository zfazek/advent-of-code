use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut va = Vec::new();
    let mut vb = Vec::new();
    let mut mb: HashMap<i64, i64> = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once("   ").unwrap();
        va.push(a.parse::<i64>().unwrap());
        let b = b.parse::<i64>().unwrap();
        vb.push(b);
        *mb.entry(b).or_default() += 1;
    }
    va.sort();
    vb.sort();
    let result1: i64 = std::iter::zip(&va, vb).map(|(a, b)| (a - b).abs()).sum();
    let result2: i64 = va
        .iter()
        .filter(|&x| mb.contains_key(x))
        .map(|x| x * mb.get(x).unwrap())
        .sum();
    dbg!(result1, result2);
}
