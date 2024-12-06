use std::collections::HashMap;

#[derive(PartialEq)]
enum Part {
    First,
    Second,
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut m = Vec::new();
    let mut si = 0;
    let mut sj = 0;
    for (y, line) in input.lines().enumerate() {
        let v = line.chars().collect::<Vec<_>>();
        m.push(v);
        if let Some(n) = line.find('^') {
            si = y;
            sj = n;
            m[si][sj] = '.';
        }
    }
    let result1 = foo(&m, si, sj, Part::First).unwrap();
    println!("{result1}");
    let mut result2: usize = 0;
    let size = m.len();
    for i in 0..size {
        for j in 0..size {
            if i == si && j == sj || m[i][j] != '.' {
                continue;
            }
            m[i][j] = '#';
            if foo(&m, si, sj, Part::Second).is_none() {
                result2 += 1;
            }
            m[i][j] = '.';
        }
    }
    println!("{result2}");
}

fn foo(sm: &Vec<Vec<char>>, si: usize, sj: usize, part: Part) -> Option<usize> {
    let mut m = sm.clone();
    let size = m.len() as i32;
    let mut i = si as i32;
    let mut j = sj as i32;
    let mut dir: (i32, i32) = (-1, 0);
    let mut dp = HashMap::new();
    loop {
        if m[i as usize][j as usize] == 'X' {
            let k = i * size + j;
            if let Some(&d) = dp.get(&k) {
                if d == dir {
                    return None;
                }
            }
        }
        m[i as usize][j as usize] = 'X';
        let ni = i + dir.0;
        let nj = j + dir.1;
        if ni < 0 || ni >= size || nj < 0 || nj >= size {
            if part == Part::First {
                return Some(count(&m, 'X'));
            } else {
                return Some(1);
            }
        }
        if m[ni as usize][nj as usize] == '#' {
            dir = turn(dir);
        } else {
            dp.insert(i * size + j, dir);
            i = ni;
            j = nj;
        }
    }
}

fn count(m: &Vec<Vec<char>>, c: char) -> usize {
    m.iter().flatten().filter(|&&x| x == c).count()
}

fn turn(d: (i32, i32)) -> (i32, i32) {
    if d.0 == -1 && d.1 == 0 {
        return (0, 1);
    } else if d.0 == 1 && d.1 == 0 {
        return (0, -1);
    } else if d.0 == 0 && d.1 == -1 {
        return (-1, 0);
    } else if d.0 == 0 && d.1 == 1 {
        return (1, 0);
    }
    (0, 0)
}
