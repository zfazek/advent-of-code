use std::fs;


fn main() {
    let input = fs::read_to_string("../01.txt").unwrap();
    one(&input);
    two(&input);
}

fn one(input: &String) {
    let mut sum = 0;
    for line in input.lines() {
        if let Ok(n) = line.parse::<i32>() {
            sum += n / 3 - 2;
        }
    }
    println!("{}", sum);
}

fn two(input: &String) {
    let mut sum = 0;
    for line in input.lines() {
        let mut s = 0;
        if let Ok(n) = line.parse::<i32>() {
            let mut m = n;
            while m > 0 {
                m = m / 3 - 2;
                if m > 0 {
                    s += m;
                }
            }
            sum += s;
        }
    }
    println!("{}", sum);
}

