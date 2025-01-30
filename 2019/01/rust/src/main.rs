fn main() {
    let input = std::fs::read_to_string("../01.txt").unwrap();
    one(&input);
    two(&input);
}

fn one(input: &str) {
    let sum = input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .map(|n| n / 3 - 2)
        .sum::<i32>();
    println!("{}", sum);
}

fn two(input: &str) {
    let sum = input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .map(|n| {
            let mut m = n;
            let mut s = 0;
            while m > 0 {
                m = m / 3 - 2;
                if m > 0 {
                    s += m;
                }
            }
            s
        })
        .sum::<i32>();
    println!("{}", sum);
}
