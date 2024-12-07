use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut result1 = 0;
    let mut result2 = 0;
    for line in input.lines() {
        let (value, nums) = line.split_once(": ").unwrap();
        let value = value.parse::<i64>().unwrap();
        let mut nums = nums
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<VecDeque<_>>();
        result1 += first(value, &mut nums).signum() * value;
        result2 += second(value, &mut nums).signum() * value;
    }
    println!("{result1}");
    println!("{result2}");
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
    ns[0] = (n0.to_string() + &ns[0].to_string())
        .parse::<i64>()
        .unwrap();
    let c = second(r, ns);
    ns[0] = n1;
    ns.push_front(n0);
    a + b + c
}
