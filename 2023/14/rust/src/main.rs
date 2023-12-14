use std::collections::HashMap;

const N: usize = 1000000000;

fn main() {
    let input = include_str!("../../input.txt");
    let mut ls = Vec::new();
    for line in input.lines() {
        let l = line.chars().collect::<Vec<_>>();
        ls.push(l);
    }
    let ls_orig = ls.clone();
    north(&mut ls);
    let ans1 = get_load(&ls);
    println!("{}", ans1);
    ls = ls_orig.clone();
    let mut dp = HashMap::new();
    let mut dp1 = HashMap::new();
    for n in 1..=N {
        north(&mut ls);
        west(&mut ls);
        south(&mut ls);
        east(&mut ls);
        let v = get_load(&ls);
        //println!("{}: {}", n, v);
        dp1.insert(n, v);
        let mut str = String::from("");
        for i in 0..ls.len() {
            for j in 0..ls[i].len() {
                str.push(ls[i][j]);
            }
        }
        if let Some(res) = dp.get(&str) {
            let pos = N - (N - res) / (n - res) * (n - res);
            //println!("BINGO: {} {} {} {}", n, res, pos, dp1.get(&pos).unwrap());
            let ans2 = dp1.get(&pos).unwrap();
            println!("{}", ans2);
            break;
        } else {
            dp.insert(str, n);
        }
    }
}

fn _print(ls: &Vec<Vec<char>>) {
    for i in 0..ls.len() {
        for j in 0..ls[i].len() {
            print!("{}", ls[i][j]);
        }
        println!();
    }
    println!();
}

fn north(ls: &mut Vec<Vec<char>>) {
    for i in 0..ls.len() {
        for j in 0..ls[i].len() {
            if ls[i][j] == 'O' {
                let mut c = 0;
                for k in (0..i).rev() {
                    if ls[k][j] == '.' {
                        c += 1;
                    }
                    if ls[k][j] == '#' {
                        break;
                    }
                }
                ls[i][j] = '.';
                ls[i - c][j] = 'O';
            }
        }
    }
}

fn south(ls: &mut Vec<Vec<char>>) {
    for i in (0..ls.len()).rev() {
        for j in 0..ls[i].len() {
            if ls[i][j] == 'O' {
                let mut c = 0;
                for k in i + 1..ls.len() {
                    if ls[k][j] == '.' {
                        c += 1;
                    }
                    if ls[k][j] == '#' {
                        break;
                    }
                }
                ls[i][j] = '.';
                ls[i + c][j] = 'O';
            }
        }
    }
}

fn east(ls: &mut Vec<Vec<char>>) {
    for i in 0..ls.len() {
        for j in (0..ls[i].len()).rev() {
            if ls[i][j] == 'O' {
                let mut c = 0;
                for k in j + 1..ls[i].len() {
                    if ls[i][k] == '.' {
                        c += 1;
                    }
                    if ls[i][k] == '#' {
                        break;
                    }
                }
                ls[i][j] = '.';
                ls[i][j + c] = 'O';
            }
        }
    }
}

fn west(ls: &mut Vec<Vec<char>>) {
    for i in 0..ls.len() {
        for j in 0..ls[i].len() {
            if ls[i][j] == 'O' {
                let mut c = 0;
                for k in (0..j).rev() {
                    if ls[i][k] == '.' {
                        c += 1;
                    }
                    if ls[i][k] == '#' {
                        break;
                    }
                }
                ls[i][j] = '.';
                ls[i][j - c] = 'O';
            }
        }
    }
}

fn get_load(ls: &Vec<Vec<char>>) -> usize {
    let mut load: usize = 0;
    for i in 0..ls.len() {
        for j in 0..ls[i].len() {
            if ls[i][j] == 'O' {
                let d = ls.len() - i;
                load += d;
                //println!("{} {}, c: {}, d: {}", i, j, c, d);
            }
        }
    }
    load
}
