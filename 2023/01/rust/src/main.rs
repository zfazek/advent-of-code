fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", foo1(input));
    println!("{}", foo2(input));
}

fn foo1(input: &str) -> usize {
    let mut c: usize = 0;
    for line in input.lines() {
        let num1 = line
            .chars().find(|&x| x.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;
        let num2 = line
            .chars()
            .filter(|&x| x.is_ascii_digit())
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;
        c += num1 * 10 + num2;
    }
    c
}

fn foo2(input: &str) -> u32 {
    let nums1 = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut c = 0;
    for line in input.lines() {
        let mut ds = Vec::new();
        for (i, v) in line.chars().enumerate() {
            if let Some(d) = v.to_digit(10) {
                ds.push(d);
            }
            for (idx, &v) in nums1.iter().enumerate() {
                if line[i..].starts_with(v) {
                    ds.push((idx + 1) as u32);
                }
            }
        }
        c += ds[0] * 10 + ds[ds.len() - 1];
    }
    c
}
