use std::collections::HashSet;

fn main() {
    let file = std::fs::read_to_string("../06.txt").unwrap();
    foo(file.as_bytes(), 4);
    foo(file.as_bytes(), 14);
}

fn foo(line: &[u8], n: usize) {
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
