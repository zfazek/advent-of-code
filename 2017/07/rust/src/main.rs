use std::collections::{HashMap, HashSet};

struct Disc {
    weight: i32,
    sub_discs: Vec<String>,
}

fn main() {
    let input = std::fs::read_to_string("../07.txt").unwrap();
    one(&input);
}

fn one(input: &String) {
    let mut sub_discs = std::collections::HashSet::new();
    let mut discs = HashMap::new();
    for line in input.lines() {
        let mut tokens = line.split(" -> ");
        let mut left_tokens = tokens.next().unwrap().split(" ");
        let mut disc_names = Vec::new();
        if let Some(right_tokens) = tokens.next() {
            for disc_name in right_tokens.split(", ") {
                sub_discs.insert(disc_name.to_string());
                disc_names.push(disc_name.to_string());
            }
        }
        let name = left_tokens.next().unwrap().to_string();
        let w = left_tokens.next().unwrap();
        let weight = str::parse::<i32>(&w[1..w.len() - 1]).unwrap();
        let disc = Disc {
            weight,
            sub_discs: disc_names.clone(),
        };
        discs.insert(name, disc);
    }
    for (disc_name, _) in &discs {
        if let Some(_) = sub_discs.get(disc_name) {
        } else {
            println!("{}\n", disc_name);
            two(&discs);
            break;
        }
    }
}

fn two(discs: &HashMap<String, Disc>) {
    for (disc_name, disc) in discs {
        let mut sub_disc_weights = HashSet::new();
        for d in &disc.sub_discs {
            let weight = foo(discs, &d);
            sub_disc_weights.insert(weight);
        }
        if sub_disc_weights.len() != 1 {
            for sub_disc in &disc.sub_discs {
                let weight = foo(discs, &sub_disc);
                println!("{}: {}, root disc: {} ({})", disc_name, weight, &sub_disc, discs.get(sub_disc).unwrap().weight);
            }
        }
        if sub_disc_weights.len() > 1 {
            println!("");
        }
    }
}

fn foo(discs: &HashMap<String, Disc>, d: &String) -> i32 {
    let disc = discs.get(d).unwrap();
    let mut sum = disc.weight;
    for i in &disc.sub_discs {
        sum += foo(discs, i);
    }
    sum
}
