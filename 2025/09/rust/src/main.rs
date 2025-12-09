fn main() {
    let input = include_str!("../../input.txt");
    let result1 = input
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .enumerate()
        .map(|(i, pi)| {
            input
                .lines()
                .map(|line| line.split_once(",").unwrap())
                .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
                .skip(i + 1)
                .map(|pj| (pi.0 - pj.0 + 1) * (pi.1 - pj.1 + 1))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap();
    println!("{}", result1);
}
