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
    let mut solutions = BTreeSet::new();
    let mut len = 0;
    for (_, v) in dp.iter() {
        for i in (2..v.len()).rev() {
            let mut found = false;
            let combinations = v.iter().combinations(i);
            for it in combinations {
                let mut it_set = BTreeSet::new();
                for &itt in it.iter() {
                    it_set.insert(itt.to_owned());
                }
                let mut good = true;
                for &j in it.iter() {
                    let vv = dp.get(j).unwrap();
                    if !vv.is_superset(&it_set) {
                        good = false;
                        break;
                    }
                }
                if good {
                    found = true;
                    if it.len() > len {
                        solutions.insert(it.clone());
                        len = it.len();
                    }
                    break;
                }
            }
            if found {
                break;
            }
        }
    }
    let mut a = solutions.iter().map(|x| (x.len(), x)).collect::<Vec<_>>();
    a.reverse();
    let b = a[0].1.iter().join(",");
    print!("{:?}", b);
    dbg!(a);
}
