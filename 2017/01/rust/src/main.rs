fn main() {
    let input = std::fs::read_to_string("../01.txt").unwrap();
    one(&input);
    two(&input);
}

fn one(line: &String) {
    let chars: Vec<_> = line.chars().collect();
    let sum = line
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let idx = if i == chars.len() - 1 { 0 } else { i + 1 };
            (chars[idx], c)
        })
        .filter(|(cc, c)| c == cc)
        .map(|c| c.0.to_digit(10).unwrap())
        .sum::<u32>();
    println!("{}", sum);
}

fn two(line: &String) {
    let chars: Vec<_> = line.chars().collect();
    let sum = (0..chars.len() / 2)
        .map(|i| {
            let c = line.chars().nth(i).unwrap();
            let idx = i + chars.len() / 2;
            (chars[idx], c)
        })
        .filter(|(cc, c)| c == cc)
        .map(|c| 2 * c.0.to_digit(10).unwrap())
        .sum::<u32>();
    println!("{}", sum);
}
