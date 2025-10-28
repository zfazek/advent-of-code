fn main() {
    let input = std::fs::read_to_string("../02.txt").unwrap();
    one(&input);
    two(&input);
}

fn one(input: &str) {
    let sum: i32 = input
        .lines()
        .map(|line| {
            let (min, max) = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .fold((i32::MAX, i32::MIN), |(min, max), n| {
                    (min.min(n), max.max(n))
                });
            max - min
        })
        .sum();
    println!("{}", sum);
}

fn two(input: &str) {
    let sum: i32 = input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            for &a in &nums {
                for &b in &nums {
                    if a != b && a % b == 0 {
                        return a / b;
                    }
                }
            }
            0
        })
        .sum();
    println!("{}", sum);
}
