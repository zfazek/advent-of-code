use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    let result1 = input
        .lines()
        .map(|line| {
            let mut tokens = line.split_ascii_whitespace();
            let lights_str = tokens.next().unwrap();
            let mut lights = Vec::new();
            lights_str.chars().for_each(|c| {
                if c == '.' {
                    lights.push(0);
                } else if c == '#' {
                    lights.push(1);
                }
            });
            let mut buttons_str = tokens.collect::<Vec<_>>();
            let _joltage = buttons_str.pop().unwrap();
            let mut buttons = Vec::new();
            for b in buttons_str.iter() {
                let b = &b[1..b.len() - 1];
                let tokens_b = b
                    .split(',')
                    .map(|t| t.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                buttons.push(tokens_b);
            }
            let powerset = buttons.iter().powerset().collect::<Vec<_>>();
            powerset
                .iter()
                .filter(|ss| {
                    let mut lights = lights.clone();
                    for &s in ss.iter() {
                        for &s1 in s.iter() {
                            lights[s1] += 1;
                        }
                    }
                    lights.iter().all(|l| l % 2 == 0)
                })
                .map(|x| x.len())
                .min()
                .unwrap()
        })
        .sum::<usize>();
    println!("{result1}");
}
