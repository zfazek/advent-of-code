fn main() {
    let input = include_str!("01.txt");
    println!("{}", foo1(&input));
    println!("{}", foo2(&input));
}


fn foo1(input: &str) -> usize {
    let mut c: usize = 0;
    for line in input.lines() {
        let num1: usize = line.chars().filter(|&x| x >= '0' && x <= '9').next().unwrap().to_digit(10).unwrap() as usize;
        let num2: usize = line.chars().filter(|&x| x >= '0' && x <= '9').last().unwrap().to_digit(10).unwrap() as usize;
        c += num1 * 10 + num2;
    }
    c
}

fn foo2(input: &str) -> usize {
    let nums1 = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let nums2 = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut c: usize = 0;
    for line in input.lines() {
        let mut num_min = 0;
        let mut num_max = 0;
        let mut min_idx = usize::MAX;
        let mut max_idx = usize::MIN;
        for (i, &v) in nums1.iter().enumerate() {
            if let Some(idx) = line.find(v) {
                if idx < min_idx {
                    min_idx = idx;
                    num_min = i + 1;
                }
            }
            if let Some(idx) = line.rfind(v) {
                if idx >= max_idx {
                    max_idx = idx;
                    num_max = i + 1;
                }
            }
        }
        for (i, &v) in nums2.iter().enumerate() {
            if let Some(idx) = line.find(v) {
                if idx < min_idx {
                    min_idx = idx;
                    num_min = i + 1;
                }
            }
            if let Some(idx) = line.rfind(v) {
                if idx >= max_idx {
                    max_idx = idx;
                    num_max = i + 1;
                }
            }
        }
        c += num_min * 10 + num_max;
    }
    c
}
