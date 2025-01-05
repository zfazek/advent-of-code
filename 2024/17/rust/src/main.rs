use num_format::{Locale, ToFormattedString};
const INPUT: &str = "Register A: 46337277
Register B: 0
Register C: 0

Program: 2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0";

const INPUTB: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0 ";

// Program: 2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0
fn main() {
    let input = INPUT;
    first(input);
    second(input);
}

fn first(input: &str) {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let registers = registers.lines().collect::<Vec<_>>();
    let ip: usize = 0;
    let a: usize = registers[0].split_once(": ").unwrap().1.parse().unwrap();
    let b: usize = registers[1].split_once(": ").unwrap().1.parse().unwrap();
    let c: usize = registers[2].split_once(": ").unwrap().1.parse().unwrap();
    let program: Vec<usize> = program
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    let out = fun_program(ip, a, b, c, &program);
    for (i, c) in out.chars().enumerate() {
        print!("{}", c);
        if i < out.len() - 1 {
            print!(",");
        }
    }
    println!();
}

fn second(input: &str) {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let registers = registers.lines().collect::<Vec<_>>();
    let ip: usize = 0;
    let b: usize = registers[1].split_once(": ").unwrap().1.parse().unwrap();
    let c: usize = registers[2].split_once(": ").unwrap().1.parse().unwrap();
    let prog_str = program
        .trim()
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>();
    let program: Vec<usize> = program
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut dp = Vec::new();
    for i in 0..8 {
        dp.push(i as usize);
    }
    loop {
        let dp1 = dp.clone();
        dp.clear();
        for n in dp1.iter() {
            for i in n * 8..n * 8 + 8 {
                let out = fun_program(ip, i, b, c, &program);
                if prog_str.ends_with(&out) {
                    dp.push(i);
                    println!(
                        "{}: {} {}",
                        i.to_formatted_string(&Locale::en),
                        out,
                        prog_str
                    );
                }
                //println!("{}", out);
                if out == prog_str {
                    println!("{}", i);
                    return;
                }
            }
        }
    }
}

fn fun_program(ip: usize, a: usize, b: usize, c: usize, program: &Vec<usize>) -> String {
    let mut ip = ip;
    let mut a = a;
    let mut b = b;
    let mut c = c;
    let mut out = String::new();
    loop {
        if ip >= program.len() - 1 {
            break;
        }
        let opcode = program[ip];
        let operand = program[ip + 1];
        if opcode == 0 {
            a = dv(a, b, c, operand);
        } else if opcode == 6 {
            b = dv(a, b, c, operand);
        } else if opcode == 7 {
            c = dv(a, b, c, operand);
        } else if opcode == 1 {
            b ^= operand;
        } else if opcode == 2 {
            b = combo(a, b, c, operand) % 8;
        } else if opcode == 3 {
            if a > 0 {
                ip = operand;
                continue;
            }
        } else if opcode == 4 {
            b ^= c;
        } else if opcode == 5 {
            out = out.to_owned() + &(combo(a, b, c, operand) % 8).to_string();
        }
        ip += 2;
    }
    out
}

fn combo(a: usize, b: usize, c: usize, operand: usize) -> usize {
    if operand < 4 {
        return operand;
    } else if operand == 4 {
        return a;
    } else if operand == 5 {
        return b;
    } else if operand == 6 {
        return c;
    }
    0
}

fn dv(a: usize, b: usize, c: usize, operand: usize) -> usize {
    let base: usize = 2;
    a / base.pow(combo(a, b, c, operand) as u32)
}
