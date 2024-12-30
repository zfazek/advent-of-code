use std::collections::BTreeMap;

struct Dir {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
struct Box {
    y: i32,
    x1: i32,
    x2: i32,
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
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
        for (j, &c) in v.iter().enumerate() {
            if c == '@' {
                robot.x = i as i32;
                robot.y = j as i32;
            }
        }
        vv.push(v);
    }
    let mut moves = Vec::new();
    for line in a.1.lines() {
        for c in line.chars() {
            moves.push(c);
        }
    }
    for m in moves {
        mov1(&mut vv, m, dirs, &mut robot);
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
    if c == '#' {
        return;
    } else if c == '.' {
        vv[y][x] = '.';
        vv[ny][nx] = '@';
        robot.x = nx as i32;
        robot.y = ny as i32;
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
                robot.x = j as i32;
                robot.y = i as i32;
            }
        }
    }
    let mut moves = Vec::new();
    for line in a.1.lines() {
        for c in line.chars() {
            moves.push(c);
        }
    }
    for m in moves {
        mov2(&mut vv, m, dirs, &mut robot);
        //_print(&vv, m, &robot);
    }
    let result2 = calculate_part2(vv);
    println!("{}", result2);
}

fn mov2(vv: &mut Vec<Vec<char>>, m: char, dirs: &BTreeMap<char, Dir>, robot: &mut Robot) {
    let dir = dirs.get(&m).unwrap();
    let mut nx = robot.x + dir.x;
    let ny = robot.y + dir.y;
    let x = robot.x as usize;
    let y = robot.y as usize;
    let c = vv[ny as usize][nx as usize];
    if c == '#' {
        return;
    } else if c == '.' {
        vv[y][x] = '.';
        vv[ny as usize][nx as usize] = '@';
        robot.x = nx;
        robot.y = ny;
        return;
    }
    let mut moving_boxes = Vec::new();
    if dir.y == 0 {
        loop {
            let c = vv[ny as usize][nx as usize];
            if c == '[' {
                let b = Box {
                    x1: nx,
                    x2: nx + 1,
                    y: ny,
                };
                moving_boxes.push(b);
            } else if c == ']' {
                let b = Box {
                    x1: nx - 1,
                    x2: nx,
                    y: ny,
                };
                moving_boxes.push(b);
            } else if c == '#' || c == '.' {
                break;
            }
            nx += dir.x * 2;
        }
        let collision = moving_boxes.iter().any(|b| {
            vv[b.y as usize][(b.x1 + dir.x) as usize] == '#'
                || vv[b.y as usize][(b.x2 + dir.x) as usize] == '#'
        });
        if collision {
            return;
        }
        for b in moving_boxes {
            vv[b.y as usize][(b.x1 + dir.x) as usize] = '[';
            vv[b.y as usize][(b.x2 + dir.x) as usize] = ']';
        }
    } else {
        let c = vv[ny as usize][nx as usize];
        if c == '[' {
            let b = Box {
                x1: nx,
                x2: nx + 1,
                y: ny,
            };
            moving_boxes.push(b);
            foo(dir.y, nx, ny, &mut moving_boxes, vv);
        } else if c == ']' {
            let b = Box {
                x1: nx - 1,
                x2: nx,
                y: ny,
            };
            moving_boxes.push(b);
            foo(dir.y, nx - 1, ny, &mut moving_boxes, vv);
        }
        moving_boxes.sort();
        if dir.y == 1 {
            moving_boxes.reverse();
        }
        let collision = moving_boxes.iter().any(|b| {
            vv[(b.y + dir.y) as usize][b.x1 as usize] == '#'
                || vv[(b.y + dir.y) as usize][b.x2 as usize] == '#'
        });
        if collision {
            return;
        }
        for b in moving_boxes {
            vv[(b.y + dir.y) as usize][(b.x1 + dir.x) as usize] = '[';
            vv[(b.y + dir.y) as usize][(b.x2 + dir.x) as usize] = ']';
            vv[b.y as usize][(b.x1 + dir.x) as usize] = '.';
            vv[b.y as usize][(b.x2 + dir.x) as usize] = '.';
        }
    }
    robot.x += dir.x;
    robot.y += dir.y;
    vv[y][x] = '.';
    vv[robot.y as usize][robot.x as usize] = '@';
}

fn foo(dir_y: i32, x: i32, y: i32, moving_boxes: &mut Vec<Box>, vv: &Vec<Vec<char>>) {
    let a1 = vv[(y + dir_y) as usize][x as usize];
    let a2 = vv[(y + dir_y) as usize][(x + 1) as usize];
    if a1 == '[' {
        let b = Box {
            y: y + dir_y,
            x1: x,
            x2: x + 1,
        };
        moving_boxes.push(b);
        foo(dir_y, x, y + dir_y, moving_boxes, vv);
    }
    if a1 == ']' {
        let b = Box {
            y: y + dir_y,
            x1: x - 1,
            x2: x,
        };
        moving_boxes.push(b);
        foo(dir_y, x - 1, y + dir_y, moving_boxes, vv);
    }
    if a2 == '[' {
        let b = Box {
            y: y + dir_y,
            x1: x + 1,
            x2: x + 2,
        };
        moving_boxes.push(b);
        foo(dir_y, x + 1, y + dir_y, moving_boxes, vv);
    }
}

fn calculate_part2(vv: Vec<Vec<char>>) -> usize {
    //dbg!(&vv);
    let mut result1 = 0;
    for i in 0..vv.len() {
        for j in 0..vv[i].len() {
            if vv[i][j] == '[' {
                result1 += 100 * i + j;
            }
        }
    }
    result1
}

fn _print(vv: &Vec<Vec<char>>, m: char, robot: &Robot) {
    println!("{}", m);
    for (i, v) in vv.iter().enumerate() {
        for (j, &c) in v.iter().enumerate() {
            if i == robot.y as usize && j == robot.x as usize {
                print!("@");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    println!();
}
