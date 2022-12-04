fn main() {
    let file = std::fs::read_to_string("../03.txt").unwrap();
    one(&file);
    two(&file);
}

fn get_priority(m: char) -> i32 {
    let n = m as i32;
    if n >= 97 {
        return n - 96;
    } else {
        return n - 65 + 27;
    }
}

fn one(file: &str) {
    let mut sum = 0;
    for line in file.lines() {
        let l: Vec<char> = line.chars().collect();
        'outer: for i in 0..l.len() / 2 {
            for j in l.len() / 2..l.len() {
                if l[i] == l[j] {
                    sum += get_priority(l[i]);
                    break 'outer;
                }
            }
        }
    }
    println!("{}", sum);
}

fn two(file: &str) {
    let lines = file.lines().collect::<Vec<_>>();
    let mut sum = 0;
    let mut x = 0;
    loop {
        let l1: Vec<char> = lines[x].chars().collect();
        let l2: Vec<char> = lines[x+1].chars().collect();
        let l3: Vec<char> = lines[x+2].chars().collect();
        x += 3;
        'outer: for i in 0..l1.len() {
            for j in 0..l2.len() {
                for k in 0..l3.len() {
                    if l1[i] == l2[j] && l1[i] == l3[k] {
                        sum += get_priority(l1[i]);
                        break 'outer;
                    }
                }
            }
        }
        if x >= lines.len() {
            break;
        }
    }
    println!("{}", sum);
}
