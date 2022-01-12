fn main() {
    let input = std::fs::read_to_string("../02.txt").unwrap();
    one(&input);
    two(&input);
}

fn one(input: &String) {
    let mut sum = 0;
    for line in input.lines() {
        let l: Vec<_> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let min = l.iter().min().unwrap();
        let max = l.iter().max().unwrap();
        sum += max - min;
    }
    println!("{}", sum);
}

fn two(input: &String) {
    let mut sum = 0;
    for line in input.lines() {
        let mut l: Vec<_> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        l.sort();
        l.reverse();
        for (i, n) in l.iter().enumerate() {
            for (j, m) in l.iter().enumerate() {
                if i < j {
                    if n % m == 0 {
                        sum += n / m;
                    }
                }
            }
        }
    }
    println!("{}", sum);
}
