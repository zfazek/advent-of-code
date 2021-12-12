use itertools::Itertools;
use std::fs;

fn main() {
    println!("Hello, world!");
    let input: Vec<i32> = fs::read_to_string("../01.txt")
        .unwrap()
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let result1 = input
        .iter()
        .tuple_windows()
        .filter(|(&a, &b)| a < b)
        .count();
    println!("{}", result1);
    let result2 = input
        .iter()
        .tuple_windows()
        .filter(|(&a, &b, &c, &d)| a + b + c < b + c + d)
        .count();
    println!("{}", result2);
}
