fn print(stacks: &Vec<Vec<char>>) {
    for i in 0..9 {
        if stacks[i].is_empty() {
            continue;
        }
        let v = stacks[i][stacks[i].len() - 1];
        print!("{}", v);
    }
    println!("");
}

fn main() {
    let f = std::fs::read_to_string("../05.txt").unwrap();
    let mut stacks1: Vec<Vec<char>> = Vec::new();
    let mut stacks2: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        let v1: Vec<char> = Vec::new();
        let v2: Vec<char> = Vec::new();
        stacks1.push(v1);
        stacks2.push(v2);
    }
    let mut first_part = true;
    for line in f.lines() {
        let l: Vec<char> = line.chars().collect();
        if l.is_empty() {
            first_part = false;
            for i in 0..9 {
                stacks1[i].reverse();
                stacks2[i].reverse();
            }
            continue;
        }
        if first_part {
            if l[1] == '1' {
                continue;
            }
            for i in 0..9 {
                let idx = i * 4 + 1;
                if idx > l.len() {
                    break;
                }
                let c = l[idx];
                if c != ' ' {
                    stacks1[i].push(c);
                    stacks2[i].push(c);
                }
            }
            continue;
        }
        if l[0] == 'm' {
            let mut ll = line.split(" ");
            ll.next();
            let n = ll.next().unwrap().parse::<i32>().unwrap();
            ll.next();
            let from = ll.next().unwrap().parse::<usize>().unwrap() - 1;
            ll.next();
            let to = ll.next().unwrap().parse::<usize>().unwrap() - 1;
            for _ in 0..n {
                let v = stacks1[from].pop().unwrap();
                stacks1[to].push(v);
            }
            for i in (1..=n).rev() {
                let idx = stacks2[from].len() - i as usize;
                let v = stacks2[from].remove(idx);
                stacks2[to].push(v);
            }
        }
    }
    print(&stacks1);
    print(&stacks2);
}
