use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let vv = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(": ").unwrap();
            let bb = b
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            (a.to_string(), bb)
        })
        .collect::<BTreeMap<_, _>>();
    let mut paths = BTreeSet::new();
    let mut path = Vec::new();
    let start = "you".to_string();
    path.push(start.clone());
    iter(&vv, start, &mut path, &mut paths);
    println!("{}", paths.len());
}

fn iter(
    vv: &BTreeMap<String, Vec<String>>,
    d: String,
    path: &mut Vec<String>,
    paths: &mut BTreeSet<Vec<String>>,
) {
    if let Some(last) = path.last() {
        if last == "out" {
            paths.insert(path.clone());
            return;
        }
        for e in vv.get(&d).unwrap().iter() {
            path.push(e.to_string());
            iter(vv, e.to_string(), path, paths);
            path.pop();
        }
    }
}
