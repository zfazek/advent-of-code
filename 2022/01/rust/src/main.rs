fn main() {
    let mut sums: Vec<i64> = std::fs::read_to_string("../01.txt")
        .unwrap()
        .split("\n\n")
        .map(|l| l.lines().map(|line| line.parse::<i64>().unwrap()).sum())
        .collect();
    sums.sort();
    sums.reverse();
    println!("{}", sums[0]);
    println!("{}", sums[0] + sums[1] + sums[2]);
}
