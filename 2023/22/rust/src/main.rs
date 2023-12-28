use regex::Regex;
use std::{
    cmp,
    collections::{BTreeMap, BTreeSet},
};

fn main() {
    let input = include_str!("../../input.txt");
    let re = Regex::new(r"^(.*),(.*),(.*)~(.*),(.*),(.*)").unwrap();
    let mut vs: Vec<(i32, i32, i32, i32, i32, i32)> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    for line in input.lines() {
        let x1 = re
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let y1 = re
            .captures(line)
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let z1 = re
            .captures(line)
            .unwrap()
            .get(3)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let x2 = re
            .captures(line)
            .unwrap()
            .get(4)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let y2 = re
            .captures(line)
            .unwrap()
            .get(5)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let z2 = re
            .captures(line)
            .unwrap()
            .get(6)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        max_x = cmp::max(max_x, x2);
        max_y = cmp::max(max_y, y2);
        max_z = cmp::max(max_z, z2);
        vs.push((z1, x1, y1, z2, x2, y2));
        if z1 > z2 {
            println!("{:?}", vs[vs.len() - 1]);
        }
    }
    let mut ms = BTreeMap::new();
    for i in 0..vs.len() {
        let (z1, x1, y1, z2, x2, y2) = vs[i];
        for x in x1..=x2 {
            for y in y1..=y2 {
                for z in z1..=z2 {
                    ms.insert((z, x, y), i);
                }
            }
        }
    }
    loop {
        let mut no_fall_happened = true;
        for j in 0..vs.len() {
            let mut can_fall = true;
            let (z1, x1, y1, z2, x2, y2) = vs[j];
            'foo: for x in x1..=x2 {
                for y in y1..=y2 {
                    if z1 < 2 || ms.contains_key(&(z1 - 1, x, y)) {
                        can_fall = false;
                        break 'foo;
                    }
                }
            }
            if can_fall {
                no_fall_happened = false;
                vs[j].0 -= 1;
                vs[j].3 -= 1;
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        for z in z1..=z2 {
                            ms.remove(&(z, x, y));
                            ms.insert((z - 1, x, y), j);
                        }
                    }
                }
            }
        }
        if no_fall_happened {
            break;
        }
    }
    let ms_orig = ms.clone();
    let vs_orig = vs.clone();
    let mut ans1 = 0;
    let mut bricks = BTreeSet::new();
    for i in 0..vs.len() {
        let (z1, x1, y1, z2, x2, y2) = vs[i];
        for x in x1..=x2 {
            for y in y1..=y2 {
                for z in z1..=z2 {
                    ms.remove(&(z, x, y));
                }
            }
        }
        let mut no_fall_happened = true;
        for j in 0..vs.len() {
            if i == j {
                continue;
            }
            let mut can_fall = true;
            let (z1, x1, y1, _z2, x2, y2) = vs[j];
            'foo: for x in x1..=x2 {
                for y in y1..=y2 {
                    if z1 < 2 || ms.contains_key(&(z1 - 1, x, y)) {
                        can_fall = false;
                        break 'foo;
                    }
                }
            }
            if can_fall {
                no_fall_happened = false;
                break;
            }
        }
        if no_fall_happened {
            ans1 += 1;
            bricks.insert(i);
        }
        ms = ms_orig.clone();
    }
    println!("ans: {}", ans1);
    let mut ans2 = 0;
    for i in 0..vs.len() {
        if bricks.contains(&i) {
            continue;
        }
        let (z1, x1, y1, z2, x2, y2) = vs[i];
        for x in x1..=x2 {
            for y in y1..=y2 {
                for z in z1..=z2 {
                    ms.remove(&(z, x, y));
                }
            }
        }
        let mut fallen_bricks = BTreeSet::new();
        loop {
            let mut no_fall_happened = true;
            for j in 0..vs.len() {
                if i == j {
                    continue;
                }
                let mut can_fall = true;
                let (z1, x1, y1, z2, x2, y2) = vs[j];
                'foo: for x in x1..=x2 {
                    for y in y1..=y2 {
                        if z1 < 2 || ms.contains_key(&(z1 - 1, x, y)) {
                            can_fall = false;
                            break 'foo;
                        }
                    }
                }
                if can_fall {
                    fallen_bricks.insert(j);
                    no_fall_happened = false;
                    vs[j].0 -= 1;
                    vs[j].3 -= 1;
                    for x in x1..=x2 {
                        for y in y1..=y2 {
                            for z in z1..=z2 {
                                ms.remove(&(z, x, y));
                                ms.insert((z - 1, x, y), j);
                            }
                        }
                    }
                }
            }
            if no_fall_happened {
                break;
            }
        }
        ans2 += fallen_bricks.len();
        ms = ms_orig.clone();
        vs = vs_orig.clone();
    }
    println!("ans: {}", ans2);
}
