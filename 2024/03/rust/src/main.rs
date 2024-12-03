use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut result1 = 0;
    let mut result2 = 0;
    let mut enabled = true;
    for line in input.lines() {
        let captures = re.captures_iter(line);
        for it in captures {
            let v = it.get(0).unwrap().as_str();
            if v.starts_with("mul") {
                let a = it.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let b = it.get(2).unwrap().as_str().parse::<i64>().unwrap();
                result1 += a * b;
                if enabled {
                    result2 += a * b;
                }
            } else if v.starts_with("do()") {
                enabled = true;
            } else if v.starts_with("don't()") {
                enabled = false;
            }
        }
    }
    println!("{result1}");
    println!("{result2}");
}
