use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand {
    orig: String,
    str: String,
    v: usize,
    map: BTreeMap<char, i32>,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let s0 = self.str.chars().next().unwrap();
        let o0 = other.str.chars().next().unwrap();
        let s0n = *self.map.get(&s0).unwrap();
        let o0n = *other.map.get(&o0).unwrap();
        let sn = self.map.len();
        let on = other.map.len();
        if sn != on {
            return on.cmp(&sn);
        }
        if sn == 3 {
            if s0n == 3 && o0n == 2 {
                return Ordering::Greater;
            }
            if o0n == 3 && s0n == 2 {
                return Ordering::Less;
            }
        }
        if sn == 2 {
            if s0n == 4 && o0n == 3 {
                return Ordering::Greater;
            }
            if o0n == 4 && s0n == 3 {
                return Ordering::Less;
            }
        }
        self.orig.cmp(&other.orig)
    }
}

fn main() {
    foo1();
    foo2();
}

fn compare(lhs: &char, rhs: &char, map: &BTreeMap<char, i32>) -> Ordering {
    if map.get(lhs).unwrap() == map.get(rhs).unwrap() {
        return rhs.cmp(lhs);
    }
    map.get(rhs).unwrap().cmp(map.get(lhs).unwrap())
}

fn foo1() {
    let mut ans1: usize = 0;
    let input: &str = include_str!("../../input.txt");
    let mut hands = BTreeSet::new();
    let input = input
        .replace('A', "Z")
        .replace('K', "Y")
        .replace('Q', "X")
        .replace('J', "W")
        .replace('T', "V");
    for line in input.lines() {
        let mut map: BTreeMap<char, i32> = BTreeMap::new();
        let mut tokens = line.split_ascii_whitespace();
        let mut str = tokens.next().unwrap().chars().collect::<Vec<_>>();
        for c in str.iter() {
            *map.entry(*c).or_default() += 1;
        }
        let orig = str.iter().collect::<String>();
        str.sort_unstable_by(|x, y| compare(x, y, &map));
        let str = str.iter().collect::<String>();
        let v = tokens.last().unwrap().parse::<usize>().unwrap();
        let clone = str.clone();
        if !hands.insert(Hand { orig, str, v, map }) {
            println!("{:?}", clone);
        }
    }
    assert_eq!(input.lines().count(), hands.len());
    for (i, hand) in hands.iter().enumerate() {
        ans1 += (i + 1) * hand.v;
        //println!("{} {} {:?} {}", hand.orig, hand.str, hand.map, hand.map.len());
    }
    println!("{}", ans1);
}

fn foo2() {
    let mut ans2: usize = 0;
    let input: &str = include_str!("../../input.txt");
    let mut hands = BTreeSet::new();
    let input = input
        .replace('A', "Z")
        .replace('K', "Y")
        .replace('Q', "X")
        .replace('J', "0")
        .replace('T', "V");
    for line in input.lines() {
        let mut map: BTreeMap<char, i32> = BTreeMap::new();
        let mut tokens = line.split_ascii_whitespace();
        let mut str = tokens.next().unwrap().chars().collect::<Vec<_>>();
        let orig = str.iter().collect::<String>();
        for c in str.iter() {
            *map.entry(*c).or_default() += 1;
        }
        str.sort_by(|x, y| compare(x, y, &map));
        let mut c = str[0];
        for &cc in str.iter() {
            if cc != '0' {
                c = cc;
                break;
            }
        }
        for s in str.iter_mut() {
            if s == &'0' {
                *s = c;
            }
        }
        map.clear();
        for c in str.iter() {
            *map.entry(*c).or_default() += 1;
        }
        str.sort_unstable_by(|x, y| compare(x, y, &map));
        let str = str.iter().collect::<String>();
        let v = tokens.last().unwrap().parse::<usize>().unwrap();
        let clone = str.clone();
        if !hands.insert(Hand { orig, str, v, map }) {
            println!("{:?}", clone);
        }
    }
    assert_eq!(input.lines().count(), hands.len());
    for (i, hand) in hands.iter().enumerate() {
        ans2 += (i + 1) * hand.v;
        //println!("{} {} {:?} {}", hand.orig, hand.str, hand.map, hand.map.len());
    }
    println!("{}", ans2);
}
