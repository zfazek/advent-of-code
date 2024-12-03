fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let result1 = first(&input);
    println!("{}", result1);
    let result2 = second(&input);
    println!("{}", result2);
}

fn first(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let mut v = Vec::new();
        let tokens = line.split_ascii_whitespace();
        for n in tokens {
            let a = n.parse::<i32>().unwrap();
            v.push(a);
        }
        if is_safe(&v) {
            result += 1;
        }
    }
    result
}

fn second(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let mut v = Vec::new();
        let tokens = line.split_whitespace();
        for n in tokens {
            let a = n.parse::<i32>().unwrap();
            v.push(a);
        }
        if is_safe(&v) {
            result += 1;
            continue;
        }
        for i in 0..v.len() {
            let vv = v.clone();
            v.remove(i);
            if is_safe(&v) {
                result += 1;
                break;
            }
            v = vv;
        }
    }
    result
}

fn is_safe(v: &Vec<i32>) -> bool {
    let sign = v[1] - v[0];
    for i in 1..v.len() {
        let d = v[i] - v[i - 1];
        if sign > 0 {
            if !(1..=3).contains(&d) {
                return false;
            }
        } else if !(-3..=-1).contains(&d) {
            return false;
        }
    }
    true
}
