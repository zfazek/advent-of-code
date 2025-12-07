use std::char;

fn main() {
    let input = include_str!("../../input.txt");
    let mut vv: Vec<Vec<usize>> = Vec::new();
    let mut grand_total1: usize = 0;
    for line in input.lines() {
        let tokens = line.split_ascii_whitespace();
        if line.contains("*") {
            let ops = tokens.collect::<Vec<_>>();
            for col in 0..vv[0].len() {
                let op = ops[col].chars().next().unwrap();
                if op == '+' {
                    let mut sum = 0;
                    (0..vv.len()).for_each(|row| {
                        sum += vv[row][col];
                    });
                    grand_total1 += sum;
                } else if op == '*' {
                    let mut prod = 1;
                    (0..vv.len()).for_each(|row| {
                        prod *= vv[row][col];
                    });
                    grand_total1 += prod;
                }
            }
        } else {
            let v: Vec<usize> = tokens
                .map(|t| t.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            vv.push(v);
        }
    }
    println!("{grand_total1}");

    let mut ops = Vec::new();
    let mut vv: Vec<Vec<char>> = Vec::new();
    let mut grand_total2: usize = 0;
    for line in input.lines() {
        if line.contains("*") {
            ops = line.chars().rev().filter(|c| *c != ' ').collect::<Vec<_>>();
        }
        let mut v: Vec<char> = line.chars().rev().collect();
        v.push(' ');
        vv.push(v);
    }
    let mut idx = 0;
    let mut sum = 0;
    let mut prod = 1;
    for j in 0..vv[0].len() {
        let mut num = String::from("");
        let op = ops[idx];
        (0..vv.len()).for_each(|i| {
            let d = vv[i][j];
            if d.is_ascii_digit() {
                num.push(d);
            }
        });
        if !num.is_empty() {
            if op == '+' {
                sum += num.parse::<usize>().unwrap();
            } else if op == '*' {
                prod *= num.parse::<usize>().unwrap();
            }
        } else {
            idx += 1;
            grand_total2 += sum;
            if prod != 1 {
                grand_total2 += prod;
            }
            prod = 1;
            sum = 0;
        }
    }
    println!("{grand_total2}");
}
