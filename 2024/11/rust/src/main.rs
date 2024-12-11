use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../../input.txt")
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut res: BTreeMap<usize, usize> = BTreeMap::new();
    let mut vv = Vec::new();
    for &v in input.iter() {
        vv.push(v);
        *res.entry(v).or_default() += 1;
    }
    let num_blink = 75;
    for _blink in 1..=num_blink {
        dbg!(_blink);
        let mut res1 = BTreeMap::new();
        for (v, &count) in res.iter() {
            if *v == 0 {
                *res1.entry(1).or_default() += count;
            } else {
                let len = v.to_string().chars().count();
                if len % 2 == 0 {
                    let n1 = v.to_string()[..len / 2].parse::<usize>().unwrap();
                    let n2 = v.to_string()[len / 2..].parse::<usize>().unwrap();
                    *res1.entry(n1).or_default() += count;
                    *res1.entry(n2).or_default() += count;
                } else {
                    *res1.entry(*v * 2024).or_default() += count;
                }
            }
        }
        res = res1;
    }
    let mut result: usize = 0;
    for k in res.iter() {
        result += *k.1;
    }
    dbg!(result);
}

fn _print(dp: &BTreeMap<usize, BTreeSet<usize>>) {
    println!();
    for (&i, v) in dp.iter() {
        print!("{i}: [ ");
        for e in v {
            print!("{} ", e);
        }
        println!("]");
    }
    println!();
}
