fn main() {
    let file = std::fs::read_to_string("../02.txt").unwrap();
    one(&file);
    two(&file);
}

fn one(file: &str) {
    let mut sum = 0;
    for line in file.lines() {
        let l: Vec<char> = line.chars().collect();
        let a = l[0] as i32 - 'A' as i32;
        let b = l[2] as i32 - 'X' as i32;
        sum += b + 1;
        if a == b {
            sum += 3;
        } else {
            if b == 0 && a == 2 {
                sum += 6;
                continue;
            } 
            if b == 1 && a == 0 {
                sum += 6;
                continue;
            }
            if b == 2 && a == 1 {
                sum += 6;
            }
        }
    }
    println!("{}", sum);
}

fn two(file: &str) {
    let mut sum = 0;
    for line in file.lines() {
        let l: Vec<char> = line.chars().collect();
        let a = l[0] as i32 - 'A' as i32;
        let mut b = l[2] as i32 - 'X' as i32;
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
        } else {
            if b == 0 && a == 2 {
                sum += 6;
                continue;
            } 
            if b == 1 && a == 0 {
                sum += 6;
                continue;
            }
            if b == 2 && a == 1 {
                sum += 6;
            }
        }
    }
    println!("{}", sum);
}
