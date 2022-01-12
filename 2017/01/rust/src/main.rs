fn main() {
    let input: Vec<_> = std::fs::read_to_string("../01.txt")
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect();
    one(&input);
    two(&input);
}

fn one(input: &Vec<String>) {
    for line in input {
        let mut sum = 0;
        let chars: Vec<_> = line.chars().collect();
        for (i, c) in line.chars().enumerate() {
            let idx = if i == chars.len() - 1 { 0 } else { i + 1 };
            if c == chars[idx] {
                sum += c.to_digit(10).unwrap();
            }
        }
        println!("{}", sum);
    }
}

fn two(input: &Vec<String>) {
    for line in input {
        let mut sum = 0;
        let chars: Vec<_> = line.chars().collect();
        for (i, c) in line.chars().enumerate() {
            if i == chars.len() / 2 {
                break;
            }
            let idx = i + chars.len() / 2;
            if c == chars[idx] {
                sum += 2 * c.to_digit(10).unwrap();
            }
        }
        println!("{}", sum);
    }
}
