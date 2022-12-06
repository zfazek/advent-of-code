use std::collections::HashSet;

fn main() {
    let file = std::fs::read_to_string("../06.txt").unwrap();
    let line: Vec<char> = file.chars().collect();
    foo(&line, 4);
    foo(&line, 14);
}

fn foo(line: &Vec<char>, n: usize) {
    for i in 0..line.len() - n - 1 {
        let mut letters = HashSet::new();
        for j in 0..n {
            letters.insert(line[i + j]);
        }
        if letters.len() == n {
            println!("{}", i + n);
            break;
        }
    }
}
