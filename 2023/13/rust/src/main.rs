use std::collections::HashMap;

fn main() {
    let mut dp = HashMap::new();
    let input = include_str!("../../input.txt");
    foo1(input, &mut dp);
    foo2(input, &mut dp);
}

fn foo1(input: &str, dp: &mut HashMap<usize, usize>) {
    let problems = input.split("\n\n");
    let mut ans1: usize = 0;
    for (p, problem) in problems.enumerate() {
        let mut ls = Vec::new();
        for line in problem.lines() {
            ls.push(line.to_string());
        }
        let res = get_row_number(&ls);
        if !res.is_empty() {
            let ans = res[0];
            dp.insert(p, 100 * ans);
            ans1 += 100 * ans;
        } else {
            //println!("ROTATE");
            let mut rs = Vec::new();
            for x in 0..ls[0].chars().count() {
                let mut s = String::from("");
                for y in 0..ls.len() {
                    let ch = ls[y].chars().nth(x).unwrap();
                    //println!("{} {} {}", x, y, ls[y]);
                    s.push(ch);
                }
                rs.push(s);
            }
            //_print(&ls);
            //_print(&rs);
            let res = get_row_number(&rs);
            if !res.is_empty() {
                let ans = res[0];
                dp.insert(p, ans);
                ans1 += ans;
            } else {
                println!("PROBLEM");
            }
        }
    }
    println!("{}", ans1);
}

fn foo2(input: &str, dp: &mut HashMap<usize, usize>) {
    let problems = input.split("\n\n");
    let mut ans2: usize = 0;
    'foo: for (p, problem) in problems.enumerate() {
        let mut ls = Vec::new();
        for line in problem.lines() {
            ls.push(line.to_string());
        }
        let ls_orig = ls.clone();
        //_print(&ls_orig);
        for i in 0..ls.len() {
            for j in 0..ls[i].len() {
                ls = ls_orig.clone();
                //let s_orig = String::from(&ls[i]);
                let mut s = String::from(&ls[i][0..j]);
                let ch = ls[i].chars().nth(j).unwrap();
                if ch == '.' {
                    s.push('#');
                } else {
                    s.push('.');
                }
                s.push_str(&ls[i][j + 1..]);
                ls[i] = s;
                let res = get_row_number(&ls);
                for &ans in res.iter() {
                    //println!("{}: {:?}", p, res);
                    //println!("{}\n{} {} {}, ans: {}", s_orig, ls[i], i, j, ans);
                    if let Some(dp) = dp.get(&p) {
                        if *dp != 100 * ans {
                            //println!("{} {} {}", i, j, 100 * ans);
                            ans2 += 100 * ans;
                            continue 'foo;
                        }
                    }
                }
            }
        }
        for i in 0..ls.len() {
            for j in 0..ls[i].len() {
                ls = ls_orig.clone();
                //let s_orig = String::from(&ls[i]);
                let mut s = String::from(&ls[i][0..j]);
                let ch = ls[i].chars().nth(j).unwrap();
                if ch == '.' {
                    s.push('#');
                } else {
                    s.push('.');
                }
                s.push_str(&ls[i][j + 1..]);
                ls[i] = s;
                //println!("{}\n{} {} {}", s_orig, ls[i], i, j);
                //println!("ROTATE");
                let mut rs = Vec::new();
                for x in 0..ls[0].chars().count() {
                    let mut s = String::from("");
                    for y in 0..ls.len() {
                        let ch = ls[y].chars().nth(x).unwrap();
                        //println!("{} {} {}", x, y, ls[y]);
                        s.push(ch);
                    }
                    rs.push(s);
                }
                let res = get_row_number(&rs);
                for &ans in res.iter() {
                    if let Some(dp) = dp.get(&p) {
                        if *dp != ans {
                            //println!("{} {} {}", i, j, ans);
                            ans2 += ans;
                            continue 'foo;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans2);
}

fn _print(ls: &Vec<String>) {
    println!();
    for l in ls {
        println!("{}", l);
    }
    println!();
}

fn get_row_number(ls: &Vec<String>) -> Vec<usize> {
    //_print(ls);
    let mut ans = Vec::new();
    for i in 1..ls.len() {
        let k = i.min(ls.len() - i);
        let mut good = true;
        for j in 0..k {
            //println!("len: {}, i: {}, k: {}, j: {}", ls.len(), i, k, j);
            let l1 = &ls[i - j - 1];
            let l2 = &ls[i + j];
            //println!("{}", l1);
            //println!("{}", l2);
            if l1 != l2 {
                good = false;
                break;
            }
        }
        if good {
            ans.push(i);
        }
    }
    ans
}
