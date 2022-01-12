fn main() {
    let mut input: Vec<i32> = std::fs::read_to_string("../05.txt")
        .unwrap()
        .lines()
        .map(|x| str::parse::<i32>(x).unwrap())
        .collect();
    one(&mut input);
    two(&mut input);
}

fn one(input: &mut Vec<i32>) {
    let size = input.len();
    let mut i = 0;
    let mut n = 0;
    loop {
        let jump = input[i];
        input[i] += 1;
        n += 1;
        if jump < 0 && i < i32::abs(jump) as usize {
            break;
        }
        if jump > 0 && i + jump as usize >= size {
            break;
        }
        if jump < 0 {
            i -= i32::abs(jump) as usize;
        } else {
            i += i32::abs(jump) as usize;
        }
    }
    println!("{}", n);
}

fn two(input: &mut Vec<i32>) {
    let size = input.len();
    let mut i = 0;
    let mut n = 0;
    loop {
        let jump = input[i];
        if jump < 3 {
            input[i] += 1;
        } else {
            input[i] -= 1;
        }
        n += 1;
        if jump < 0 && i < i32::abs(jump) as usize {
            break;
        }
        if jump > 0 && i + jump as usize >= size {
            break;
        }
        if jump < 0 {
            i -= i32::abs(jump) as usize;
        } else {
            i += i32::abs(jump) as usize;
        }
    }
    println!("{}", n);
}
