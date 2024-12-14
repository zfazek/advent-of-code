use regex::Regex;
use std::collections::BTreeSet;

const X: i32 = 101;
const Y: i32 = 103;
const N: usize = 100;
//const X: i32 = 11;
//const Y: i32 = 7;
//const N: usize = 100;

#[derive(Debug, Clone)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = Vec::new();
    for line in input.lines() {
        let capture = re.captures(line).unwrap();
        let x = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let dx = capture.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let dy = capture.get(4).unwrap().as_str().parse::<i32>().unwrap();
        robots.push(Robot { x, y, dx, dy });
    }
    let robots1 = robots.clone();
    for _ in 0..N {
        update_robot_positions(&mut robots);
    }
    let result1 = get_result1(&robots);
    println!("{}", result1);
    robots = robots1;
    for n in 1.. {
        update_robot_positions(&mut robots);
        if check_christmas_tree(&robots) {
            println!("{n}");
            break;
        }
        if n > 1000000000 {
            break;
        }
    }
}

fn check_christmas_tree(robots: &[Robot]) -> bool {
    let mut dp = BTreeSet::new();
    for r in robots {
        dp.insert((r.x, r.y));
    }
    for r in robots {
        let mut good = true;
        let x = r.x;
        let y = r.y;
        for n in 1..9 {
            let x1 = x + n;
            if x1 >= X {
                good = false;
                break;
            }
            if !dp.contains(&(x1, y)) {
                good = false;
                break;
            }
        }
        if good {
            //print_hq(robots);
            return true;
        }
    }
    false
}

fn _print_hq(robots: &[Robot]) {
    let mut hq = vec!['.'; (X * Y).try_into().unwrap()];
    for r in robots {
        let pos: usize = (r.y * X + r.x).try_into().unwrap();
        hq[pos] = '#';
    }
    for y in 0..Y {
        for x in 0..X {
            let pos: usize = (y * X + x).try_into().unwrap();
            print!("{}", hq[pos]);
        }
        println!();
    }
    println!();
    println!();
}

fn update_robot_positions(robots: &mut [Robot]) {
    for r in robots.iter_mut() {
        let mut x = r.x + r.dx;
        let mut y = r.y + r.dy;
        if x < 0 {
            x += X;
        } else if x >= X {
            x -= X;
        }
        if y < 0 {
            y += Y;
        } else if y >= Y {
            y -= Y;
        }
        r.x = x;
        r.y = y;
    }
}

fn get_result1(robots: &Vec<Robot>) -> i32 {
    let mut res = [0, 0, 0, 0];
    let x2 = X / 2;
    let y2 = Y / 2;
    for r in robots.iter() {
        let x = r.x;
        let y = r.y;
        if x == x2 || y == y2 {
            continue;
        }
        if x < x2 && y < y2 {
            res[0] += 1;
        }
        if x > x2 && y < y2 {
            res[1] += 1;
        }
        if x < x2 && y > y2 {
            res[2] += 1;
        }
        if x > x2 && y > y2 {
            res[3] += 1;
        }
    }
    res.iter().product::<i32>()
}
