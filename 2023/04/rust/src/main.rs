use std::collections::BTreeMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let mut f = Foo::new();
    let ans = f.foo(input);
    println!("{}", ans.0);
    println!("{}", ans.1);
}

struct Foo {
    dp: BTreeMap<i32, usize>,
    cards: BTreeMap<i32, usize>,
}

impl Foo {
    pub fn new() -> Self {
        let dp = BTreeMap::new();
        let cards = BTreeMap::new();
        Foo { dp, cards }
    }

    fn foo(&mut self, input: &str) -> (usize, usize) {
        for line in input.lines() {
            let card = line
                .split(" | ")
                .next()
                .unwrap()
                .split(": ")
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            self.cards.insert(card, 1);
            let winning = line
                .split(" | ")
                .next()
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split_ascii_whitespace()
                .collect::<HashSet<_>>();
            let other = line
                .split(" | ")
                .last()
                .unwrap()
                .split_ascii_whitespace()
                .collect::<HashSet<_>>();
            self.dp.insert(card, winning.intersection(&other).count());
        }
        let ans1: usize = self
            .cards
            .iter()
            .map(|e| 2_usize.pow(*self.dp.get(e.0).unwrap() as u32) / 2 * e.1)
            .sum();
        for e in self.cards.clone().iter() {
            self.iter(*e.0);
        }
        let ans2: usize = self.cards.values().sum();
        (ans1, ans2)
    }

    fn iter(&mut self, n: i32) {
        for i in 1..=*self.dp.get(&n).unwrap() {
            let card_num = n + i as i32;
            *self.cards.get_mut(&card_num).unwrap() += 1;
            self.iter(card_num);
        }
    }
}
