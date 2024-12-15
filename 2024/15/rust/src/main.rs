use std::collections::BTreeMap;

struct Dir {
    x: i32,
    y: i32,
}

struct Robot {
    x: i32,
    y: i32,
}

fn main() {
    let input = std::fs::read_to_string("../inputb.txt").unwrap();
    let mut dirs = BTreeMap::new();
    dirs.insert('<', Dir { x: -1, y: 0 });
    dirs.insert('>', Dir { x: 1, y: 0 });
    dirs.insert('^', Dir { x: 0, y: -1 });
    dirs.insert('v', Dir { x: 0, y: 1 });
    first(&input, &dirs);
    second(&input, &dirs);
}

fn first(input: &str, dirs: &BTreeMap<char, Dir>) {
    let a = input.split_once("\n\n").unwrap();
    let mut vv = Vec::new();
    let mut robot = Robot { x: 0, y: 0 };
    for (i, line) in a.0.lines().enumerate() {
        let v = line.chars().collect::<Vec<_>>();
        for j in 0..v.len() {
            let c = v[j];
            if c == '@' {
                robot.x = i as i32;
                robot.y = j as i32;
            }
        }
        vv.push(v);
    }
    //_print(&vv, ' ');
    let mut moves = Vec::new();
    for line in a.1.lines() {
        for c in line.chars() {
            moves.push(c);
        }
    }
    for m in moves {
        mov1(&mut vv, m, &dirs, &mut robot);
        //_print(&vv, m);
    }
    let result1 = calculate_part1(vv);
    println!("{}", result1);
}

fn mov1(vv: &mut Vec<Vec<char>>, m: char, dirs: &BTreeMap<char, Dir>, robot: &mut Robot) {
    let dir = dirs.get(&m).unwrap();
    let nx = (robot.x + dir.x) as usize;
    let ny = (robot.y + dir.y) as usize;
    let x = robot.x as usize;
    let y = robot.y as usize;
    let c = vv[ny][nx];
    if c == '.' {
        vv[y][x] = '.';
        vv[ny][nx] = '@';
        robot.x = nx as i32;
        robot.y = ny as i32;
        return;
    } else if c == '#' {
        return;
    }
    let mut k = 2;
    loop {
        let nx1 = (robot.x + k * dir.x) as usize;
        let ny1 = (robot.y + k * dir.y) as usize;
        let c = vv[ny1][nx1];
        if c == '#' {
            return;
        }
        if c == '.' {
            vv[y][x] = '.';
            vv[ny][nx] = '@';
            vv[ny1][nx1] = 'O';
            robot.x = nx as i32;
            robot.y = ny as i32;
            return;
        }
        k += 1;
    }
}

fn calculate_part1(vv: Vec<Vec<char>>) -> usize {
    let mut result1 = 0;
    for i in 0..vv.len() {
        for j in 0..vv[i].len() {
            if vv[i][j] == 'O' {
                result1 += 100 * i + j;
            }
        }
    }
    result1
}

fn second(input: &str, dirs: &BTreeMap<char, Dir>) {
    let a = input.split_once("\n\n").unwrap();
    let mut vv = Vec::new();
    for line in a.0.lines() {
        let mut v = Vec::new();
        for c in line.chars() {
            if c == '@' {
                v.push('@');
                v.push('.');
            } else if c == '#' {
                v.push('#');
                v.push('#');
            } else if c == 'O' {
                v.push('[');
                v.push(']');
            } else if c == '.' {
                v.push('.');
                v.push('.');
            }
        }
        vv.push(v);
    }
    let mut robot = Robot { x: 0, y: 0 };
    for i in 0..vv.len() {
        for j in 0..vv[i].len() {
            if vv[i][j] == '@' {
                robot.x = i as i32;
                robot.y = j as i32;
            }
        }
    }
    _print(&vv, ' ');
    let mut moves = Vec::new();
    for line in a.1.lines() {
        for c in line.chars() {
            moves.push(c);
        }
    }
    for m in moves {
        //mov2(&mut vv, m, &dirs, &mut robot);
        //_print(&vv, m);
    }
    let result2 = calculate_part2(vv);
    println!("{}", result2);
}

fn mov2(vv: &mut Vec<Vec<char>>, m: char, dirs: &BTreeMap<char, Dir>, robot: &mut Robot) {
    let dir = dirs.get(&m).unwrap();
    let nx = (robot.x + dir.x) as usize;
    let ny = (robot.y + dir.y) as usize;
    let x = robot.x as usize;
    let y = robot.y as usize;
    let c = vv[ny][nx];
    if c == '.' {
        vv[y][x] = '.';
        vv[ny][nx] = '@';
        robot.x = nx as i32;
        robot.y = ny as i32;
        return;
    } else if c == '#' {
        return;
    }
    let mut k = 2;
    loop {
        let nx1 = (robot.x + k * dir.x) as usize;
        let ny1 = (robot.y + k * dir.y) as usize;
        let c = vv[ny1][nx1];
        if c == '#' {
            return;
        }
        if c == '.' {
            vv[y][x] = '.';
            vv[ny][nx] = '@';
            vv[ny1][nx1] = 'O';
            robot.x = nx as i32;
            robot.y = ny as i32;
            return;
        }
        k += 1;
    }
}

fn calculate_part2(vv: Vec<Vec<char>>) -> usize {
    //dbg!(&vv);
    let mut result1 = 0;
    let width = vv[0].len();
    let height = vv.len(); 
    for i in 0..vv.len() {
        for j in 0..vv[i].len() {
            if vv[i][j] == '[' {
                let x1 = j;
                let x2 = width - (j + 1);
                let y1 = i;
                let y2 = height - (i + 1);
                result1 += 100 * y1.min(y2) + x1.min(x2);
            }
        }
    }
    result1
}

fn _print(vv: &Vec<Vec<char>>, m: char) {
    println!("{}", m);
    for v in vv.iter() {
        for c in v.iter() {
            print!("{}", c);
        }
        println!();
    }
    println!();
}
