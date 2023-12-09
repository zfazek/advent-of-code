fn main() {
    let mut ans1: i64 = 0;
    let mut ans2: i64 = 0;
    let input = include_str!("../../input.txt");
    for line in input.lines() {
        let mut nums = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut first_nums = Vec::new();
        'foo: loop {
            first_nums.push(nums[0]);
            let mut nums_diff = Vec::new();
            for i in 0..nums.len() - 1 {
                nums_diff.push(nums[i + 1] - nums[i]);
            }
            ans1 += nums[nums.len() - 1] as i64;
            nums = nums_diff;
            for &n in nums.iter() {
                if n != 0 {
                    continue 'foo;
                }
            }
            break;
        }
        let mut first_num = 0;
        for i in (0..first_nums.len()).rev() {
            first_num = first_nums[i] - first_num;
        }
        ans2 += first_num as i64;
    }
    println!("{}", ans1);
    println!("{}", ans2);
}
