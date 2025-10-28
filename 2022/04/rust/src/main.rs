fn main() {
    let file = std::fs::read_to_string("../04.txt").unwrap();
    let (sum1, sum2) = file.lines().fold((0, 0), |(mut acc1, mut acc2), line| {
        let (first, second) = line.split_once(",").unwrap();
        let (a1, a2) = parse_token(first);
        let (b1, b2) = parse_token(second);
        if a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2 {
            acc1 += 1;
        }
        if a1 <= b2 && a2 >= b1 {
            acc2 += 1;
        }
        (acc1, acc2)
    });
    println!("{}", sum1);
    println!("{}", sum2);
}

fn parse_token(first: &str) -> (i32, i32) {
    let (a1, a2) = first.split_once("-").unwrap();
    let a1 = a1.parse::<i32>().unwrap();
    let a2 = a2.parse::<i32>().unwrap();
    (a1, a2)
}
