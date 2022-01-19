fn main() {
    let input = std::fs::read_to_string("../01.txt").unwrap();
    one(&input);
    two(&input);
}

fn one(input: &str) {
    let mut n = 0;
    for line in input.lines() {
        let num: i32 = line.parse().unwrap();
        n += num;
    }
    println!("{}", n);
}

fn two(input: &str) {
    let mut n = 0;
    let mut seen = std::collections::BTreeSet::new();
    seen.insert(0);
    loop {
        for line in input.lines() {
            let num: i32 = line.parse().unwrap();
            n += num;
            if seen.contains(&n) {
                println!("{}", n);
                std::process::exit(0);
            } else {
                seen.insert(n);
            }
        }
    }
}
