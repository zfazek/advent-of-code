struct Monkey {
    levels: Vec<u64>,
    op: String,
    test: u64,
    t: u32,
    f: u32,
    n: usize,
}

fn main() {
    foo(20, 3);
    foo(10000, 1);
}

fn foo(round: usize, den: u64) {
    let file = std::fs::read_to_string("../11.txt").unwrap();
    let monkeys = file.split("\n\n");
    let mut ms: Vec<Monkey> = Vec::new();
    for m in monkeys {
        let mut levels: Vec<u64> = Vec::new();
        let mut op = String::new();
        let mut test = 0;
        let mut t: u32 = 0;
        let mut f: u32 = 0;
        for line in m.lines() {
            let l: Vec<char> = line.chars().collect();
            if l[2] == 'S' {
                for (i, t) in line.split(": ").enumerate() {
                    if i < 1 {
                        continue;
                    }
                    let its: Vec<&str> = t.split(", ").collect();
                    for it in its {
                        levels.push(it.parse::<u64>().unwrap());
                    }
                }
            } else if l[2] == 'O' {
                let mut tokens = line.split("= ");
                tokens.next().unwrap();
                let o: Vec<&str> = tokens.collect();
                op = o[0].to_string();
            } else if l[2] == 'T' {
                let mut tokens = line.split("by ");
                tokens.next().unwrap();
                let o: Vec<&str> = tokens.collect();
                test = o[0].parse::<u64>().unwrap();
            } else if l[2] == ' ' && l[7] == 't' {
                t = l[29].to_digit(10).unwrap();
            } else if l[2] == ' ' && l[7] == 'f' {
                f = l[30].to_digit(10).unwrap();
            }
        }
        let n = 0;
        let monkey = Monkey{levels, op, test, t, f, n};
        ms.push(monkey);
    }
    let divider: u64 = ms.iter().map(|m| m.test).product();
    for _ in 0..round {
        for i in 0..ms.len() {
            let c1 = ms[i].op.chars().nth(4).unwrap();
            let c2 = ms[i].op.chars().nth(6).unwrap();
            let levels: Vec<u64> = ms[i].levels.drain(..).collect();
            ms[i].n += levels.len();
            for l in levels {
                let a;
                let mut level;
                if c2 == 'o' {
                    a = l;
                } else {
                    let tokens = ms[i].op.split(" ");
                    let o: Vec<&str> = tokens.collect();
                    a = o[2].parse::<u64>().unwrap();
                }
                if c1 == '+' {
                    level = l + a;
                } else {
                    level = l * a;
                }
                level = level / den;
                level %= divider;
                if level % ms[i].test == 0 {
                    let idx = ms[i].t as usize;
                    ms[idx].levels.push(level);
                } else {
                    let idx = ms[i].f as usize;
                    ms[idx].levels.push(level);
                }
            }
        }
    }
    let mut ns: Vec<usize> = (0..ms.len()).map(|i| ms[i].n).collect();
    ns.sort_by(|a, b| b.cmp(a));
    let ans: usize = ns.iter().take(2).product();
    println!("{}", ans);
}
