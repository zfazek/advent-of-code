use std::collections::BTreeSet;
use std::fs::read_to_string;

fn main() {
    let file = read_to_string("../adventofcode_202001.txt").unwrap();
    let mut numbers = BTreeSet::<i32>::new();
    for f in file.lines() {
        let n = f.parse::<i32>().unwrap();
        numbers.insert(n);
    }
    let n1 = numbers
        .iter()
        .filter(|&n| numbers.contains(&(2020 - *n)))
        .map(|&n| n * (2020 - n))
        .next()
        .unwrap();
    println!("{n1}");
    'l: for &n in numbers.iter() {
        for &n1 in numbers.iter() {
            let n2 = 2020 - n - n1;
            if numbers.contains(&n2) {
                println!("{}", n * n1 * n2);
                break 'l;
            }
        }
    }
}
