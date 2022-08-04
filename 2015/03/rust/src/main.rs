use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};

struct House {
    x: i32,
    y: i32,
}

impl Hash for House {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for House {
    fn eq(&self, other: &House) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for House {}

fn main() {
    let input = read_to_string("../03.txt").unwrap();
    one(&input);
}

fn one(input: &String) {
    let mut houses = HashSet::new();
    for line in input.lines() {
        houses.clear();
        let mut x = 0;
        let mut y = 0;
        houses.insert(House{x, y});
        for c in line.chars() {
            if c == '^' {
                y += 1;
            } else if c == 'v' {
                y -= 1;
            } else if c == '<' {
                x -= 1;
            } else if c == '>' {
                x += 1;
            }
            houses.insert(House{x, y});
        }
    }
    println!("{}", houses.len());
}
