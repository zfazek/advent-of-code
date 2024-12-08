use std::collections::BTreeSet;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let mut dp: HashMap<char, BTreeSet<(i32, i32)>> = HashMap::new();
    let mut positions1 = BTreeSet::new();
    let mut positions2 = BTreeSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                dp.entry(c).or_default().insert((i as i32, j as i32));
            }
        }
    }
    let size = input.lines().count() as i32;
    for e in dp.iter() {
        let antenna_positions = e.1;
        for p1 in antenna_positions.iter() {
            positions2.insert(*p1);
            for p2 in antenna_positions.iter() {
                if p1 == p2 {
                    continue;
                }
                let dir = (p2.0 - p1.0, p2.1 - p1.1);
                let mut new_pos = (p2.0 + dir.0, p2.1 + dir.1);
                let mut first = true;
                loop {
                    if new_pos.0 < 0 || new_pos.0 >= size || new_pos.1 < 0 || new_pos.1 >= size {
                        break;
                    }
                    if first {
                        positions1.insert(new_pos);
                        first = false;
                    }
                    positions2.insert(new_pos);
                    new_pos = (new_pos.0 + dir.0, new_pos.1 + dir.1);
                }
            }
        }
    }
    println!("{}", positions1.len());
    println!("{}", positions2.len());
}
