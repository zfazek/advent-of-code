fn main() {
    let input = include_str!("../../01.txt");
    imperative(input);
    let vv = input.chars().collect::<Vec<_>>();
    let n = foo(&vv, 0, 0);
    let idx = foo1(&vv, 0, 0);
    println!("{}", n);
    println!("{}", idx);
}

fn imperative(input: &str) {
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

fn foo(vv: &Vec<char>, i: usize, count: i32) -> i32 {
    if i == vv.len() {
        return count;
    }
    if vv[i] == '(' {
        return foo(vv, i + 1, count + 1);
    } else {
        return foo(vv, i + 1, count - 1);
    }
}

fn foo1(vv: &Vec<char>, i: usize, count: i32) -> usize {
    if vv[i] == '(' {
        return foo1(vv, i + 1, count + 1);
    } else {
        if count < 0 {
            return i;
        }
        return foo1(vv, i + 1, count - 1);
    }
}
