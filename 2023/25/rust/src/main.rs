use graphs::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

use crate::graphs::min_cut::MinimumCut;

mod graphs;

fn main() {
    let input = include_str!("../../input.txt");
    let mut edges = BTreeSet::new();
    for line in input.lines() {
        let (name, components) = line.split_once(": ").unwrap();
        let vs = components
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        edges.insert(name.to_string());
        for e in vs {
            let ee = e.clone();
            edges.insert(ee);
        }
    }
    let mut map = BTreeMap::new();
    let mut rev_map = BTreeMap::new();
    let mut i: usize = 0;
    for e in edges.iter() {
        map.insert(e.clone(), i);
        rev_map.insert(i, e.clone());
        i += 1;
    }
    let mut gs: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for line in input.lines() {
        let (name, components) = line.split_once(": ").unwrap();
        let vs = components
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        for v in vs.iter() {
            gs.entry(name.to_string())
                .or_default()
                .insert(v.to_string());
        }
        for v in vs.iter() {
            gs.entry(v.to_string())
                .or_default()
                .insert(name.to_string());
        }
    }
    let mut input: Vec<Vec<usize>> = Vec::new();
    for e in gs.iter() {
        let mut v = Vec::new();
        v.push(*map.get(e.0).unwrap());
        for n in e.1 {
            v.push(*map.get(n).unwrap());
        }
        input.push(v);
    }
    let g = Graph::import_edges(&input).expect("Error: Couldn't load input edges");
    let mc = g.minimum_cut().unwrap();
    println!("{:?}", mc);
    for e in mc.edges {
        let i = e.1.iter().next().unwrap();
        match i {
            NodeType::N(n) => {
                let n1 = rev_map.get(&e.0).unwrap();
                let n2 = rev_map.get(&n).unwrap();
                println!("{} {}", n1, n2);
                gs.get_mut(n1).unwrap().remove(n2);
                gs.get_mut(n2).unwrap().remove(n1);
            }
            _ => todo!(),
        }
    }
    let n = get_max(edges.iter().next().unwrap(), &gs);
    let len = edges.len();
    println!("{} {}", len, n * (len - n));
}

fn get_max(name: &str, gs: &BTreeMap<String, BTreeSet<String>>) -> usize {
    let mut visited: BTreeSet<String> = BTreeSet::new();
    let mut q = VecDeque::new();
    q.push_back(name);
    visited.insert(name.to_string());
    let mut counter = 0;
    while let Some(n) = q.pop_front() {
        counter += 1;
        for e in gs.get(n).unwrap() {
            if !visited.contains(e) {
                visited.insert(e.to_string());
                q.push_back(e);
            }
        }
    }
    counter
}
