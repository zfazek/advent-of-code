use rayon::prelude::*;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};
// 029A
// 980A
// 179A
// 456A
// 379A
//
// 140A
// 180A
// 176A
// 805A
// 638A
fn main() {
    let input = include_str!("../../input.txt");
    let mut numeric = BTreeMap::new();
    numeric.insert(('0', '2'), "^A");
    numeric.insert(('0', '5'), "^^A");
    numeric.insert(('0', 'A'), ">A");
    numeric.insert(('1', '4'), "^A");
    numeric.insert(('1', '7'), "^^A");
    numeric.insert(('1', '8'), "^^>A");
    numeric.insert(('2', '9'), ">^^A");
    numeric.insert(('3', '7'), "<<^^A");
    numeric.insert(('3', '8'), "<^^A");
    numeric.insert(('4', '0'), ">vvA");
    numeric.insert(('4', '5'), ">A");
    numeric.insert(('5', '6'), ">A");
    numeric.insert(('5', 'A'), "vv>A");
    numeric.insert(('6', '3'), "vA");
    numeric.insert(('6', 'A'), "vvA");
    numeric.insert(('7', '6'), "v>>A");
    numeric.insert(('7', '9'), ">>A");
    numeric.insert(('8', '0'), "vvvA");
    numeric.insert(('8', 'A'), "vvv>A");
    numeric.insert(('9', '8'), "<A");
    numeric.insert(('9', 'A'), "vvvA");
    numeric.insert(('A', '0'), "<A");
    numeric.insert(('A', '1'), "^<<A");
    numeric.insert(('A', '3'), "^A");
    numeric.insert(('A', '4'), "^^<<A");
    numeric.insert(('A', '6'), "^^A");
    numeric.insert(('A', '8'), "<^^^A");
    numeric.insert(('A', '9'), "^^^A");
    let mut directional = BTreeMap::new();
    directional.insert(('<', '>'), ">>A");
    directional.insert(('<', 'A'), ">>^A");
    directional.insert(('<', '^'), ">^A");
    directional.insert(('<', 'v'), ">A");
    directional.insert(('>', '<'), "<<A");
    directional.insert(('>', 'A'), "^A");
    directional.insert(('>', '^'), "<^A");
    directional.insert(('>', 'v'), "<A");
    directional.insert(('A', '<'), "v<<A");
    directional.insert(('A', '>'), "vA");
    directional.insert(('A', '^'), "<A");
    directional.insert(('A', 'v'), "<vA");
    directional.insert(('^', '<'), "v<A");
    directional.insert(('^', '>'), "v>A");
    directional.insert(('^', 'A'), ">A");
    directional.insert(('^', 'v'), "vA");
    directional.insert(('v', '<'), "<A");
    directional.insert(('v', '>'), ">A");
    directional.insert(('v', 'A'), "^>A");
    directional.insert(('v', '^'), "^A");

    let times = 25;
    let dp: BTreeMap<(char, char), usize> = BTreeMap::new();
    let dpa = Arc::new(Mutex::new(dp));
    let num = Arc::new(Mutex::new(0));
    directional.par_iter().for_each(|d| {
        let mut str1 = directional.get(d.0).unwrap().to_string();
        for _ in 0..times - 1 {
            str1 = foo(&directional, &str1);
        }
        let mut dp1 = dpa.lock().unwrap();
        dp1.insert(*d.0, str1.len());
        let mut num = num.lock().unwrap();
        *num += 1;
        println!("{}/20", *num);
    });
    let mut result1: usize = 0;
    for line in input.lines() {
        let n: usize = line[..3].parse().unwrap();
        let mut pc = 'A';
        let mut result: usize = 0;
        for c in line.chars() {
            let str: String = numeric.get(&(pc, c)).unwrap().to_string();
            let mut p = 'A';
            for s in str.chars() {
                if p == s {
                    result += 1;
                } else {
                    let dp = dpa.lock().unwrap();
                    result += *dp.get(&(p, s)).unwrap();
                }
                p = s;
            }
            pc = c;
        }
        result1 += n * result;
    }
    println!("{}", result1);
}

fn foo(directional: &BTreeMap<(char, char), &str>, str1: &str) -> String {
    let mut p = 'A';
    let mut str = String::new();
    for c in str1.chars() {
        //println!("{} {}", p, c);
        if p == c {
            str += "A";
        } else {
            str += directional.get(&(p, c)).unwrap();
        }
        p = c;
    }
    str
}
