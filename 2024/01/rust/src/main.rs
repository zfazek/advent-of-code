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
    let mut result1 = 0;
    let mut result2 = 0;
    for (i, n) in va.iter().enumerate() {
        result1 += (n - vb[i]).abs();
        if mb.contains_key(n) {
            result2 += n * mb.get(n).unwrap();
        }
    }
    println!("{}", result1);
    println!("{}", result2);
}
