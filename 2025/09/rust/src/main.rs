fn main() {
    let ps = include_str!("../../input.txt")
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()));
    let result1 = ps
        .clone()
        .enumerate()
        .map(|(i, pi)| {
            ps.clone()
                .skip(i + 1)
                .map(|pj| (pi.0 - pj.0 + 1) * (pi.1 - pj.1 + 1))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap();
    println!("{}", result1);
}
