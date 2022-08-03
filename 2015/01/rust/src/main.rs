fn main() {
    let input = std::fs::read_to_string("../01.txt").unwrap();
    let mut n:i64 = 0;
    let mut idx:i64 = 0;
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c == '(' {
                n += 1;
            } else if c == ')' {
                n -= 1;
                    if n < 0 && idx == 0 {
                        idx = i as i64 + 1;
                    }
            }
        }
    }
    println!("{}", n);
    println!("{}", idx);
}
