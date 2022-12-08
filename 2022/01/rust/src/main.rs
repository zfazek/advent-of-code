fn main() {
    let mut sums: Vec<i32> = std::fs::read_to_string("../01.txt")
        .unwrap()
        .split("\n\n")
        .map(|l| l.lines().map(|line| line.parse::<i32>().unwrap()).sum())
        .collect();
    sums.sort_by(|a, b| b.cmp(a));
    let ans2: i32 = sums.iter().take(3).sum();
    println!("{}", sums[0]);
    println!("{}", ans2);
}
