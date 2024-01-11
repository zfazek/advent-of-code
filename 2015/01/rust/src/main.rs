fn main() {
    let input = include_str!("../../01.txt");
    let mut n = 0;
    let mut idx = 0;
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '(' {
                n += 1;
            } else if c == ')' {
                n -= 1;
                if n < 0 && idx == 0 {
                    idx = i + 1;
                }
            }
        }
    }
    println!("{}", n);
    println!("{}", idx);
}
