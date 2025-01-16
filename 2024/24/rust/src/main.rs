use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("../../input.txt");
    let (a, b) = input.split_once("\n\n").unwrap();
    let mut dp: BTreeMap<String, u32> = BTreeMap::new();
    let mut regs = BTreeSet::new();
    let mut prog = Vec::new();
    for line in a.lines() {
        let (r, v) = line.split_once(": ").unwrap();
        dp.insert(r.to_string(), v.parse::<u32>().unwrap());
    }
    for line in b.lines() {
        let (c, r) = line.split_once(" -> ").unwrap();
        let mut it = c.split_ascii_whitespace();
        let a = it.next().unwrap().to_string();
        let o = it.next().unwrap().to_string();
        let b = it.next().unwrap().to_string();
        let first = a.to_owned().min(b.to_owned());
        let second = a.to_owned().max(b.to_owned());
        prog.push((first, o.to_owned(), second, r.to_string()));
        regs.insert(a);
        regs.insert(b);
        regs.insert(r.to_string());
    }
    //dbg!(&regs.len());
    //dbg!(&prog.len());
    let mut idx = Vec::new();
    for i in 0..prog.len() {
        idx.push(i);
    }
    let mut x = String::new();
    let mut y = String::new();
    for (r, v) in dp.iter() {
        if r.starts_with("x") {
            x += &v.to_string();
        }
        if r.starts_with("y") {
            y += &v.to_string();
        }
    }
    x = x.chars().rev().collect::<String>();
    y = y.chars().rev().collect::<String>();
    //let x = usize::from_str_radix(&x, 2).unwrap();
    //let y = usize::from_str_radix(&y, 2).unwrap();
    println!("x:  {}", x);
    println!("y:  {}", y);
    //dbg!(combinations.count());
    while dp.len() < regs.len() {
        for (a, o, b, r) in prog.iter() {
            if dp.contains_key(a) && dp.contains_key(b) {
                let a = dp.get(a).unwrap();
                let b = dp.get(b).unwrap();
                if o == "AND" {
                    dp.insert(r.to_owned(), a & b);
                } else if o == "OR" {
                    dp.insert(r.to_owned(), a | b);
                } else {
                    dp.insert(r.to_owned(), a ^ b);
                }
            } else {
                continue;
            }
        }
    }
    let mut z = String::new();
    //dbg!(&dp);
    for (r, v) in dp.iter() {
        if r.starts_with("z") {
            z += &v.to_string();
        }
    }
    z = z.chars().rev().collect::<String>();
    /*
    println!("z: {}", z);
    */
    let result1 = usize::from_str_radix(&z, 2).unwrap();
    println!("z: {}", result1);
    let mut dp2: BTreeMap<usize, BTreeSet<(String, String, String, String)>> = BTreeMap::new();
    for i in 0..z.len() {
        let mut dp1 = VecDeque::new();
        if i < 10 {
            dp1.push_back("z0".to_owned() + &i.to_string());
        } else {
            dp1.push_back("z".to_owned() + &i.to_string());
        }
        while let Some(e) = dp1.pop_front() {
            for p in prog.iter() {
                if p.3 == e {
                    dp1.push_back(p.0.to_owned());
                    dp1.push_back(p.2.to_owned());
                    dp2.entry(i).or_default().insert(p.to_owned());
                }
            }
        }
        if i < 10 {
            print!("z0");
        } else {
            print!("z");
        }
        let foo = dp2.get(&i).unwrap().iter().filter(|x| x.1 == "OR").count();
        println!(
            "{}: {:3?}/{} {:?}",
            i,
            dp2.get(&i).unwrap().len(),
            foo,
            dp2.get(&i).unwrap()
        );
    }
    for i in 0..dp2.len() - 1 {
        let pp = dp2.get(&i).unwrap();
        for p in pp.iter() {
            if !p.3.starts_with("z") && !dp2.get(&(i + 1)).unwrap().contains(p) {
                println!("{} {:?} {:?}", i, p, dp2.get(&(i + 1)).unwrap());
            }
        }
    }
}
