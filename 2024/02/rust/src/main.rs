fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let result1 = first(&input);
    println!("{result1}");
    let result2 = second(&input);
    println!("{result2}");
}

fn first(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let tokens = line.split_ascii_whitespace();
        let v = tokens
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if is_safe(&v) {
            result += 1;
        }
    }
    result
}

fn second(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let tokens = line.split_whitespace();
        let v = tokens
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if is_safe(&v) {
            result += 1;
            continue;
        }
        for i in 0..v.len() {
            let mut vv = v.clone();
            vv.remove(i);
            if is_safe(&vv) {
                result += 1;
                break;
            }
        }
    }
    result
}

fn is_safe(v: &Vec<i32>) -> bool {
    let vv = v.clone();
    v.windows(2).all(|a| (1..=3).contains(&(a[0] - a[1])))
        || vv.windows(2).all(|a| (1..=3).contains(&(a[1] - a[0])))
}
