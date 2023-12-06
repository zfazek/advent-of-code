static INPUT: &str = include_str!("../../input.txt");

fn main() {
    println!("{}", foo1(INPUT));
    println!("{}", foo2(INPUT));
}

fn foo1(input: &str) -> i64 {
    let mut ans: i64 = 1;
    let lines = input.lines().collect::<Vec<_>>();
    let times = lines[0].split_ascii_whitespace().collect::<Vec<_>>();
    let distances = lines[1].split_ascii_whitespace().collect::<Vec<_>>();
    for i in 1..times.len() {
        let time = times[i].parse::<i32>().unwrap();
        let distance = distances[i].parse::<i32>().unwrap();
        let mut count: i64 = 0;
        for n in 1..time {
            if (time - n) * n > distance {
                count += 1;
            }
        }
        ans *= count;
    }
    ans
}

fn foo2(input: &str) -> i64 {
    let mut ans: i64 = 0;
    let lines = input.lines().collect::<Vec<_>>();
    let time = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse::<i64>()
        .unwrap();
    let distance = lines[1]
        .split(':')
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse::<i64>()
        .unwrap();
    for n in 1..time {
        if (time - n) * n > distance {
            ans += 1;
        }
    }
    ans
}
