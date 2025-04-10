fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let result1 = first(&input);
    println!("{result1}");
    let result2 = second(&input);
    println!("{result2}");
}

fn first(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(is_safe)
        .count()
}

fn second(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|v| is_safe(v) || is_safe2(v))
        .count()
}

fn is_safe(v: &Vec<i32>) -> bool {
    v.windows(2).all(|a| (1..=3).contains(&(a[0] - a[1])))
        || v.windows(2).all(|a| (1..=3).contains(&(a[1] - a[0])))
}

fn is_safe2(v: &Vec<i32>) -> bool {
    (0..v.len())
        .map(|i| {
            let mut vv = v.clone();
            vv.remove(i);
            vv
        })
        .any(|v| is_safe(&v))
}
