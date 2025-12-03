fn main() {
    let input = include_str!("../../input.txt");
    [2, 12]
        .iter()
        .map(|&n| {
            input
                .lines()
                .map(|line| {
                    let mut idx = 0;
                    (0..n)
                        .rev()
                        .map(|i| {
                            let slice = &line[idx..line.len() - i];
                            let max_digit = slice.chars().max().unwrap();
                            idx = slice.find(max_digit).unwrap() + idx + 1;
                            max_digit
                        })
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .sum::<usize>()
        })
        .for_each(|v| println!("{v}"));
}
