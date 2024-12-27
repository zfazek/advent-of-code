fn main() {
    let input = include_str!("../../input.txt");
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for schemantic in input.split("\n\n") {
        let s = schemantic
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        if s[0][0] == '.' {
            let key = (0..s[0].len())
                .map(|j| (0..s.len()).filter(|&i| s[i][j] == '#').count() - 1)
                .collect::<Vec<_>>();
            keys.push(key);
        } else {
            let lock = (0..s[0].len())
                .map(|j| (0..s.len()).filter(|&i| s[i][j] == '#').count() - 1)
                .collect::<Vec<_>>();
            locks.push(lock);
        }
    }
    let result: usize = keys
        .iter()
        .map(|k| {
            locks
                .iter()
                .filter(|l| (0..l.len()).all(|x| k[x] + l[x] < 6))
                .count()
        })
        .sum();

    println!("{result}");
}
