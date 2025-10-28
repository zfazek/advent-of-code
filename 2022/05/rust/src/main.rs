const SIZE: usize = 9;

fn main() {
    let f = std::fs::read_to_string("../05.txt").unwrap();
    let mut stacks1 = Vec::new();
    let mut stacks2 = Vec::new();
    for _ in 0..SIZE {
        stacks1.push(Vec::new());
        stacks2.push(Vec::new());
    }
    let mut first_part = true;
    for line in f.lines() {
        let l: Vec<char> = line.chars().collect();
        if l.is_empty() {
            first_part = false;
            for i in 0..SIZE {
                stacks1[i].reverse();
                stacks2[i].reverse();
            }
            continue;
        }
        if first_part {
            if l[1] == '1' {
                continue;
            }
            for i in 0..SIZE {
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
            let ll: Vec<&str> = line.split(" ").collect();
            let (n, from, to) = (
                ll[1].parse::<usize>().unwrap(),
                ll[3].parse::<usize>().unwrap() - 1,
                ll[5].parse::<usize>().unwrap() - 1,
            );
            for _ in 0..n {
                let v = stacks1[from].pop().unwrap();
                stacks1[to].push(v);
            }
            for i in (1..=n).rev() {
                let idx = stacks2[from].len() - i;
                let v = stacks2[from].remove(idx);
                stacks2[to].push(v);
            }
        }
    }
    println!("{}", get_tops(&stacks1));
    println!("{}", get_tops(&stacks2));
}

fn get_tops(stacks: &[Vec<char>]) -> String {
    stacks.iter().take(SIZE).filter_map(|x| x.last()).collect()
}
