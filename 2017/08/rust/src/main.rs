use std::collections::HashMap;

// b inc 5 if a > 1
// a inc 1 if b < 5
// c dec -10 if a >= 1
// c inc -20 if c == 10

fn main() {
    let mut registers = HashMap::new();
    let input = std::fs::read_to_string("../08.txt").unwrap();
    let mut highest = 0;
    for line in input.lines() {
        let mut tokens = line.split(" ");
        let register = tokens.next().unwrap();
        if registers.get(register) == None {
            registers.insert(register, 0);
        }
        let cmd = tokens.next().unwrap();
        let operand = str::parse::<i32>(tokens.next().unwrap()).unwrap();
        let cmd_if = tokens.next().unwrap();
        assert!(cmd_if == "if");
        let cond_first = tokens.next().unwrap();
        let cond = tokens.next().unwrap();
        let cond_second = tokens.next().unwrap();
        if registers.get(cond_first) == None {
            registers.insert(cond_first, 0);
        }
        if registers.get(cond_second) == None {
            if let Ok(v) = str::parse::<i32>(cond_second) {
                registers.insert(cond_second, v);
            } else {
                registers.insert(cond_second, 0);
            }
        }
        let cond1 = registers.get(cond_first).unwrap();
        let cond2 = registers.get(cond_second).unwrap();
        match cond {
            ">" => {
                if cond1 <= cond2 {
                    continue;
                }
            }
            ">=" => {
                if cond1 < cond2 {
                    continue;
                }
            }
            "<" => {
                if cond1 >= cond2 {
                    continue;
                }
            }
            "<=" => {
                if cond1 > cond2 {
                    continue;
                }
            }
            "==" => {
                if cond1 != cond2 {
                    continue;
                }
            }
            "!=" => {
                if cond1 == cond2 {
                    continue;
                }
            }
            _ => {
                panic!("");
            }
        }
        if cmd == "inc" {
            let new_value = registers.get(register).unwrap() + operand;
            if new_value > highest {
                highest = new_value;
            }
            registers.insert(register, new_value);
        } else {
            let new_value = registers.get(register).unwrap() - operand;
            if new_value > highest {
                highest = new_value;
            }
            registers.insert(register, new_value);
        }
    }
    let max = *registers
        .iter()
        .filter(|(&register, _)| str::parse::<i32>(register).is_err())
        .map(|x| x.1)
        .max()
        .unwrap();
    println!("{}", max);
    println!("{}", highest);
}
