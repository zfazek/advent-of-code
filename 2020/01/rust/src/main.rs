use std::collections::HashSet;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let mut numbers = HashSet::<i32>::new();
    let file = read_to_string("../adventofcode_202001.txt").unwrap();
    for f in file.lines() {
        let n = f.parse::<i32>().unwrap();
        numbers.insert(n);
    }
    for n in &numbers {
        let n1 = 2020 - n;
        if numbers.contains(&n1) {
            println!("{}", n * n1);
            break;
        }
    }
    for n in &numbers {
        for n1 in &numbers {
            let n2 = 2020 - n - n1;
            if numbers.contains(&n2) {
                println!("{}", n * n1 * n2);
                exit(0);
            }
        }
    }
}
