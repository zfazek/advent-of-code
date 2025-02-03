use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let results = input
        .lines()
        .map(|line| {
            let (value, nums) = line.split_once(": ").unwrap();
            let value = value.parse::<i64>().unwrap();
            let mut nums = nums
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<VecDeque<_>>();
            (
                first(value, &mut nums).signum() * value,
                second(value, &mut nums).signum() * value,
            )
        })
        .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1))
        .unwrap();
    println!("{}", results.0);
    println!("{}", results.1);
}

fn first(r: i64, ns: &mut VecDeque<i64>) -> i64 {
    if ns[0] > r {
        return 0;
    }
    if ns.len() == 1 {
        if ns[0] == r {
            return 1;
        } else {
            return 0;
        }
    }
    let n0 = ns.pop_front().unwrap();
    ns[0] += n0;
    let a = first(r, ns);
    ns[0] -= n0;
    ns[0] *= n0;
    let b = first(r, ns);
    ns[0] /= n0;
    ns.push_front(n0);
    a + b
}

fn second(r: i64, ns: &mut VecDeque<i64>) -> i64 {
    if ns[0] > r {
        return 0;
    }
    if ns.len() == 1 {
        if ns[0] == r {
            return 1;
        } else {
            return 0;
        }
    }
    let n0 = ns.pop_front().unwrap();
    ns[0] += n0;
    let a = second(r, ns);
    ns[0] -= n0;
    ns[0] *= n0;
    let b = second(r, ns);
    ns[0] /= n0;
    let n1 = ns[0];
    ns[0] = format!("{}{}", n0, &ns[0]).parse::<i64>().unwrap();
    let c = second(r, ns);
    ns[0] = n1;
    ns.push_front(n0);
    a + b + c
}
