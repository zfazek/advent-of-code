use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    let mut dp: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let input = include_str!("../../input.txt");
    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        dp.entry(a.to_owned()).or_default().insert(b.to_owned());
        dp.entry(b.to_owned()).or_default().insert(a.to_owned());
        dp.entry(a.to_owned()).or_default().insert(a.to_owned());
        dp.entry(b.to_owned()).or_default().insert(b.to_owned());
    }
    let mut vv = BTreeSet::new();
    for (k, ss) in dp.iter() {
        for v in ss.iter() {
            if k == v {
                continue;
            }
            for w in ss.iter() {
                if v == w {
                    continue;
                }
                if w == k {
                    continue;
                }
                if dp.get(v).unwrap().contains(w) {
                    let mut v = vec![k, v, w];
                    v.sort();
                    vv.insert(v);
                }
            }
        }
    }
    let result1 = vv
        .iter()
        .filter(|x| x[0].starts_with('t') || x[1].starts_with('t') || x[2].starts_with('t'))
        .count();
    println!("{result1}");
    let mut computers = Vec::new();
    for s in dp.keys() {
        computers.push(s.to_owned());
    }
    let lans = dp.values().collect::<BTreeSet<_>>();
    //dbg!(&ss);
    let it = computers.iter().combinations(4);
}
