use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../../input.txt");
    first(input);
    second(input);
}

fn first(input: &str) {
    let mut result1: usize = 0;
    for line in input.lines() {
        let mut n = line.parse::<usize>().unwrap();
        for _ in 0..2000 {
            n = calculate(n);
        }
        result1 += n;
    }
    println!("{result1}");
}

fn second(input: &str) {
    let mut changes = BTreeSet::new();
    let mut ccc = Vec::new();
    let mut vvv = Vec::new();
    for line in input.lines() {
        let mut cc: Vec<i32> = Vec::new();
        let c = line.chars().last().unwrap().to_digit(10).unwrap();
        cc.push(c as i32);
        let mut n = line.parse::<usize>().unwrap();
        for _ in 0..2000 {
            n = calculate(n);
            let c = n.to_string().chars().last().unwrap().to_digit(10).unwrap();
            cc.push(c as i32);
        }
        let mut vv = Vec::new();
        for i in 1..cc.len() {
            vv.push(cc[i] - cc[i - 1]);
        }
        ccc.push(cc);
        for i in 0..vv.len() - 3 {
            changes.insert((vv[i], vv[i + 1], vv[i + 2], vv[i + 3]));
        }
        vvv.push(vv);
    }
    let mut result2: usize = 0;
    for c in changes.iter() {
        let mut res: usize = 0;
        for i in 0..vvv.len() {
            let v = &vvv[i];
            for j in 0..v.len() - 3 {
                if v[j] == c.0 && v[j + 1] == c.1 && v[j + 2] == c.2 && v[j + 3] == c.3 {
                    res += ccc[i][j + 4] as usize;
                    break;
                }
            }
        }
        if res > result2 {
            result2 = res;
        }
    }
    println!("{result2}");
}

fn calculate(n: usize) -> usize {
    let mut secret = n;
    let n1 = secret * 64;
    secret ^= n1;
    secret = prune(secret);
    let n1 = secret / 32;
    secret ^= n1;
    secret = prune(secret);
    let n1 = secret * 2048;
    secret ^= n1;
    secret = prune(secret);
    secret
}

fn prune(secret: usize) -> usize {
    let mut n = secret;
    if n == 100000000 {
        16113920
    } else {
        n %= 16777216;
         n
    }
}
