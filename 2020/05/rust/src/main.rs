use std::{collections, fs};

// FFFBBFBLLR

fn main() {
    let mut ids = collections::HashSet::<i32>::new();
    let contents = fs::read_to_string("../adventofcode_202005.txt").unwrap();
    let max = contents
        .lines()
        .map(|line| {
            let num = i32::from_str_radix(&transform(&line), 2).unwrap();
            ids.insert(num);
            num
        })
        .max()
        .unwrap();
    println!("{}", max);
    for i in 1..1024 {
        if ids.contains(&(i - 1))
            && !ids.contains(&i)
            && ids.contains(&(i + 1))
        {
            println!("{}", i);
            break;
        }
    }
}

fn transform(line: &str) -> String {
    let result = line
        .chars()
        .map(|c| if c == 'B' || c == 'R' { '1' } else { '0' })
        .collect::<String>();
    result
}
