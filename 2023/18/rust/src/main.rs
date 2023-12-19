use std::collections::{BTreeSet, HashMap};
use std::io::Write;

fn main() {
    let input = include_str!("../../inputa.txt");
    foo1(input);
    _foo2(input);
}

fn foo1(input: &str) {
    let mut ans1: i64 = 0;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut prev_d = 'U';
    let mut xs = BTreeSet::new();
    let mut ys = BTreeSet::new();
    for line in input.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let d = tokens.next().unwrap().chars().next().unwrap();
        let n = tokens.next().unwrap().parse::<i64>().unwrap();
        let mut dx: i64 = 0;
        let mut dy: i64 = 0;
        if d == 'R' {
            dx = 1;
            dy = 0;
        } else if d == 'L' {
            dx = -1;
            dy = 0;
        } else if d == 'D' {
            dx = 0;
            dy = 1;
        } else if d == 'U' {
            dx = 0;
            dy = -1;
        }
        if prev_d == 'U' {
            xs.insert(x);
            ys.insert(y);
        } else if prev_d == 'D' {
            xs.insert(x);
            ys.insert(y);
        }
        if prev_d == 'R' {
            xs.insert(x);
            ys.insert(y);
        } else if prev_d == 'L' {
            xs.insert(x);
            ys.insert(y);
        }
        x += dx;
        y += dy;
        for _ in 0..n - 1 {
            xs.insert(x);
            ys.insert(y);
            x += dx;
            y += dy;
        }
        prev_d = d;
    }
    println!("{} {}", ys.len(), xs.len());
    for py in ys {
        let mut ps = Vec::new();
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        let mut prev_d = 'U';
        for line in input.lines() {
            let mut tokens = line.split_ascii_whitespace();
            let d = tokens.next().unwrap().chars().next().unwrap();
            let n = tokens.next().unwrap().parse::<i64>().unwrap();
            let mut dx: i64 = 0;
            let mut dy: i64 = 0;
            if d == 'R' {
                dx = 1;
                dy = 0;
            } else if d == 'L' {
                dx = -1;
                dy = 0;
            } else if d == 'D' {
                dx = 0;
                dy = 1;
            } else if d == 'U' {
                dx = 0;
                dy = -1;
            }
            if py == y {
                if prev_d == 'U' {
                    if d == 'R' {
                        ps.push((x, 'F'));
                    } else {
                        ps.push((x, '7'));
                    }
                } else if prev_d == 'D' {
                    if d == 'R' {
                        ps.push((x, 'L'));
                    } else {
                        ps.push((x, 'J'));
                    }
                }
                if prev_d == 'R' {
                    if d == 'U' {
                        ps.push((x, 'J'));
                    } else {
                        ps.push((x, '7'));
                    }
                } else if prev_d == 'L' {
                    if d == 'D' {
                        ps.push((x, 'F'));
                    } else {
                        ps.push((x, 'L'));
                    }
                }
            }
            x += dx;
            y += dy;
            for _ in 0..n - 1 {
                if py == y {
                    if d == 'R' || d == 'L' {
                        ps.push((x, '-'));
                    } else {
                        ps.push((x, '|'));
                    }
                }
                x += dx;
                y += dy;
            }
            prev_d = d;
        }
        ps.sort();
        println!("{} {:?}", py, ps);
        let mut count = 0;
        let mut ans: i64 = 0;
        for x in 0..ps.len() {
            ans += 1;
            let c = ps[x].1;
            if vec!['|', 'L', 'J'].contains(&c) {
                count += 1;
            }
            if count % 2 == 1 {
                ans += ps[x + 1].0 - ps[x].0 - 1;
            }
        }
        ans1 += ans;
    }
    println!();
    println!();
    println!();
    println!("{}", ans1);
}

fn _foo2(input: &str) {
    let mut ans2: i64 = 0;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut xs = BTreeSet::new();
    let mut ys = BTreeSet::new();
    let mut prev_d = 'U';
    for line in input.lines() {
        let token = line.split_once("(#").unwrap().1;
        let color = token[..5].to_string();
        let mut d = token.split_once(")").unwrap().0.chars().last().unwrap();
        let n = i64::from_str_radix(&color, 16).unwrap();
        if d == '0' {
            d = 'R';
        } else if d == '1' {
            d = 'D';
        } else if d == '2' {
            d = 'L';
        } else if d == '3' {
            d = 'U';
        }
        let mut dx: i64 = 0;
        let mut dy: i64 = 0;
        if d == 'R' {
            dx = 1;
            dy = 0;
        } else if d == 'L' {
            dx = -1;
            dy = 0;
        } else if d == 'D' {
            dx = 0;
            dy = 1;
        } else if d == 'U' {
            dx = 0;
            dy = -1;
        }
        if prev_d == 'U' {
            xs.insert(x);
            ys.insert(y);
        } else if prev_d == 'D' {
            xs.insert(x);
            ys.insert(y);
        }
        if prev_d == 'R' {
            xs.insert(x);
            ys.insert(y);
        } else if prev_d == 'L' {
            xs.insert(x);
            ys.insert(y);
        }
        x += dx;
        y += dy;
        for _ in 0..n - 1 {
            xs.insert(x);
            ys.insert(y);
            x += dx;
            y += dy;
        }
        prev_d = d;
    }
    println!("{} {}", ys.len(), xs.len());
    //println!("{:?}", ys);
    let mut yy: i64 = 0;
    let ys_len = ys.len();
    for py in ys {
        yy += 1;
        if yy % 1000 == 0 {
            print!("\r{:.1}% : {}", 100.0 * yy as f32 / ys_len as f32, ans2);
            std::io::stdout().flush().unwrap();
        }
        let mut ps = Vec::new();
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        let mut prev_d = 'U';
        for line in input.lines() {
            let token = line.split_once("(#").unwrap().1;
            let color = token[..5].to_string();
            let mut d = token.split_once(")").unwrap().0.chars().last().unwrap();
            let n = i64::from_str_radix(&color, 16).unwrap();
            if d == '0' {
                d = 'R';
            } else if d == '1' {
                d = 'D';
            } else if d == '2' {
                d = 'L';
            } else if d == '3' {
                d = 'U';
            }
            let mut dx: i64 = 0;
            let mut dy: i64 = 0;
            if d == 'R' {
                dx = 1;
                dy = 0;
            } else if d == 'L' {
                dx = -1;
                dy = 0;
            } else if d == 'D' {
                dx = 0;
                dy = 1;
            } else if d == 'U' {
                dx = 0;
                dy = -1;
            }
            if py == y {
                if prev_d == 'U' {
                    if d == 'R' {
                        ps.push((x, 'F'));
                    } else {
                        ps.push((x, '7'));
                    }
                } else if prev_d == 'D' {
                    if d == 'R' {
                        ps.push((x, 'L'));
                    } else {
                        ps.push((x, 'J'));
                    }
                }
                if prev_d == 'R' {
                    if d == 'U' {
                        ps.push((x, 'J'));
                    } else {
                        ps.push((x, '7'));
                    }
                } else if prev_d == 'L' {
                    if d == 'D' {
                        ps.push((x, 'F'));
                    } else {
                        ps.push((x, 'L'));
                    }
                }
            }
            x += dx;
            y += dy;
            for _ in 0..n - 1 {
                if py == y {
                    if d == 'R' || d == 'L' {
                        ps.push((x, '-'));
                    } else {
                        ps.push((x, '|'));
                    }
                }
                x += dx;
                y += dy;
            }
            prev_d = d;
        }
        ps.sort();
        //println!("{} {:?}", py, ps);
        let mut count = 0;
        let mut ans: i64 = 0;
        for x in 0..ps.len() {
            ans += 1;
            let c = ps[x].1;
            if vec!['|', 'L', 'J'].contains(&c) {
                count += 1;
            }
            if count % 2 == 1 {
                ans += ps[x + 1].0 - ps[x].0 - 1;
            }
        }
        ans2 += ans;
    }
    println!();
    println!();
    println!();
    println!("{}", ans2);
}

fn _print(vs: &Vec<Vec<char>>) {
    for y in 0..vs.len() {
        for x in 0..vs[y].len() {
            print!("{}", vs[y][x]);
        }
        println!();
    }
    println!();
}
