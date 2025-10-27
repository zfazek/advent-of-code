fn main() {
    let file = std::fs::read_to_string("../02.txt").unwrap();
    one(&file);
    two(&file);
}

fn one(file: &str) {
    let sum: i32 = file
        .lines()
        .map(|line| {
            let l = line.as_bytes();
            let a = (l[0] - b'A') as i32;
            let b = (l[2] - b'X') as i32;
            if a == b {
                b + 4
            } else if b == 0 && a == 2 || b == 1 && a == 0 || b == 2 && a == 1 {
                b + 7
            } else {
                b + 1
            }
        })
        .sum();
    println!("{}", sum);
}

fn two(file: &str) {
    let mut sum = 0;
    for line in file.lines() {
        let l = line.as_bytes();
        let a = (l[0] - b'A') as i32;
        let mut b = (l[2] - b'X') as i32;
        if b == 0 {
            if a == 0 {
                b = 2;
            } else if a == 1 {
                b = 0;
            } else {
                b = 1;
            }
        } else if b == 1 {
            b = a;
        } else if b == 2 {
            if a == 0 {
                b = 1;
            } else if a == 1 {
                b = 2;
            } else {
                b = 0;
            }
        }
        sum += b + 1;
        if a == b {
            sum += 3;
        } else if b == 0 && a == 2 || b == 1 && a == 0 || b == 2 && a == 1 {
            sum += 6;
        }
    }
    println!("{}", sum);
}
