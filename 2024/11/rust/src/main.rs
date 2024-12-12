use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../../input.txt")
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut dp: BTreeMap<usize, usize> = BTreeMap::new();
    for &v in input.iter() {
        dp.insert(v, 1);
    }
    let num_blink = 75;
    for _blink in 1..=num_blink {
        let mut dp1 = BTreeMap::new();
        for (&v, &count) in dp.iter() {
            if v == 0 {
                *dp1.entry(1).or_default() += count;
            } else {
                let len = v.to_string().len();
                if len % 2 == 0 {
                    let n1 = v.to_string()[..len / 2].parse::<usize>().unwrap();
                    let n2 = v.to_string()[len / 2..].parse::<usize>().unwrap();
                    *dp1.entry(n1).or_default() += count;
                    *dp1.entry(n2).or_default() += count;
                } else {
                    *dp1.entry(v * 2024).or_default() += count;
                }
            }
        }
        dp = dp1;
    }
    let result: usize = dp.values().sum();
    println!("{result}");
}
