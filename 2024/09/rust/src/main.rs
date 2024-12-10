#[derive(Debug, Copy, Clone)]
struct File {
    idx: usize,
    len: usize,
    num: i32,
}

fn main() {
    let input = include_str!("../../inputa.txt").chars().collect::<Vec<_>>();
    let mut file = true;
    let mut num = 0;
    let mut idx = 0;
    let mut vv = Vec::new();
    let mut dp = Vec::new();
    for i in input.iter() {
        if let Some(len) = i.to_digit(10) {
            let len = len as usize;
            if file {
                if len > 0 {
                    dp.push(File { idx, len, num });
                    idx += len;
                    for _ in 0..len {
                        vv.push(num);
                    }
                }
                file = false;
                num += 1;
            } else {
                if len > 0 {
                    dp.push(File { idx, len, num: -1 });
                    idx += len;
                    for _ in 0..len {
                        vv.push(-1);
                    }
                }
                file = true;
            }
        }
    }
    idx = 0;
    loop {
        let c = vv.pop().unwrap();
        if c == -1 {
            continue;
        }
        idx = find1(&vv, idx);
        if idx == 0 {
            vv.push(c);
            break;
        }
        vv[idx] = c;
    }
    let mut result1: usize = 0;
    for (i, &n) in vv.iter().enumerate() {
        result1 += i * n as usize;
    }
    println!("{}", result1);
    let mut ridx = dp.len() - 1;
    loop {
        //_print(&dp);
        let f = dp[ridx];
        if f.num == -1 {
            ridx -= 1;
            continue;
        }
        idx = find2(&dp, &f, ridx);
        if idx == 0 {
            if ridx == 0 {
                break;
            }
            ridx -= 1;
            continue;
        }
        dp[ridx].num = -1;
        if dp[idx].len != f.len {
            let i = dp[idx].idx;
            dp.insert(idx, f);
            dp[idx].idx = dp[idx + 1].idx;
            dp[idx + 1].len -= dp[idx].len;
            dp[idx + 1].idx = i + dp[idx].len;
            dp[idx].num = f.num;
        } else {
            dp[idx].num = f.num;
        }
    }
    let mut result2: usize = 0;
    for f in dp.iter() {
        if f.num == -1 {
            continue;
        }
        for i in f.idx..f.idx + f.len {
            result2 += i * f.num as usize;
        }
    }
    println!("{}", result2);
    _print(&dp);
}

fn find1(vv: &Vec<i32>, idx: usize) -> usize {
    for i in idx..vv.len() {
        if vv[i] == -1 {
            return i;
        }
    }
    0
}

fn find2(vv: &Vec<File>, file: &File, ridx: usize) -> usize {
    for (i, &n) in vv.iter().enumerate() {
        if i == ridx {
            break;
        }
        if n.num == -1 && n.len >= file.len {
            return i;
        }
    }
    0
}

fn _print(vv: &Vec<File>) {
    for f in vv.iter() {
        print!("({},{},{}) ", f.idx, f.len, f.num);
    }
    println!();
    for f in vv.iter() {
        for _ in 0..f.len {
            if f.num == -1 {
                print!(".");
            } else {
                print!("{}", f.num);
            }
        }
    }
    println!();
}
