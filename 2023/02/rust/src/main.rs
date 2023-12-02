use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", foo1(&input));
    println!("{}", foo2(&input));
}

fn foo1(input: &str) -> usize {
    let mut c: usize = 0;
    let map = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    'line: for line in input.lines() {
        let idx = line
            .split(':')
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        for set in line.split(':').last().unwrap().split(';') {
            for color in set.split(',') {
                let mut it = color.split(' ');
                if it.nth(1).unwrap().parse::<usize>().unwrap()
                    > *map.get(it.next().unwrap()).unwrap()
                {
                    continue 'line;
                }
            }
        }
        c += idx as usize;
    }
    c
}

fn foo2(input: &str) -> usize {
    let mut c = 0;
    for line in input.lines() {
        let mut map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for set in line.split(':').last().unwrap().split(';') {
            for color in set.split(',') {
                let mut it = color.split(' ');
                let n = it.nth(1).unwrap().parse::<usize>().unwrap();
                let c = it.next().unwrap();
                map.insert(c, max(n, *map.get(&c).unwrap()));
            }
        }
        c += *map.get("blue").unwrap() * *map.get("green").unwrap() * *map.get("red").unwrap();
    }
    c
}
