use std::collections::BTreeSet;

fn main() {
    let (ranges, ids) = include_str!("../../input.txt").split_once("\n\n").unwrap();
    let mut fresh_ranges = BTreeSet::new();
    for range in ranges.lines() {
        let (a, b) = range.split_once("-").unwrap();
        fresh_ranges.insert((a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()));
    }
    let mut result1: usize = 0;
    for id in ids.lines() {
        let id = id.parse::<i64>().unwrap();
        if foo(id, &fresh_ranges) {
            result1 += 1;
        }
    }
    println!("{}", result1);
    //let mut ids2 = BTreeSet::new();
    let mut count = 0;
    let mut result2 = 0;
    for range in ranges.lines() {
        count += 1;
        let (a, b) = range.split_once("-").unwrap();
        let (a, b) = (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap());
        //println!("{} {}", count, range);
        result2 += b - a + 1;
    }
    //println!("{}", result2);
}

fn foo(id: i64, ranges: &BTreeSet<(i64, i64)>) -> bool {
    ranges.iter().any(|(a, b)| id >= *a && id <= *b)
}
