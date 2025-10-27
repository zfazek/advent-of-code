fn main() {
    let file = std::fs::read_to_string("../03.txt").unwrap();
    one(&file);
    two(&file);
}

fn get_priority(n: u8) -> i32 {
    if n >= 97 {
        (n - 96).into()
    } else {
        (n - 65 + 27).into()
    }
}

fn one(file: &str) {
    let mut sum = 0;
    for line in file.lines() {
        let line = line.as_bytes();
        'outer: for i in 0..line.len() / 2 {
            for j in line.len() / 2..line.len() {
                if line[i] == line[j] {
                    sum += get_priority(line[i]);
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
        let line1 = lines[x].as_bytes();
        let line2 = lines[x + 1].as_bytes();
        let line3 = lines[x + 2].as_bytes();
        x += 3;
        'outer: for i in 0..line1.len() {
            for j in 0..line2.len() {
                for k in 0..line3.len() {
                    if line1[i] == line2[j] && line1[i] == line3[k] {
                        sum += get_priority(line1[i]);
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
