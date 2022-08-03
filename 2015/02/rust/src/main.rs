fn main() {
    let lines = std::fs::read_to_string("../02.txt").unwrap();
    let mut result1:i64 = 0;
    let mut result2:i64 = 0;
    for line in lines.lines() {
        let mut nums = vec![0; 3];
        let mut tokens = line.split("x");
        let l = str::parse::<i32>(tokens.next().unwrap()).unwrap();
        let w = str::parse::<i32>(tokens.next().unwrap()).unwrap();
        let h = str::parse::<i32>(tokens.next().unwrap()).unwrap();
        nums[0] = l;
        nums[1] = w;
        nums[2] = h;
        nums.sort();
        let lw = l * w;
        let lh = l * h;
        let wh = w * h;
        let r1 = 2 * lw + 2 * lh + 2 * wh;
        result1 += r1 as i64;
        result1 += (nums[0] * nums[1]) as i64;
        let r2 = 2 * nums[0] + 2 * nums[1] + l * w * h;
        result2 += r2 as i64;
    }
    println!("{}", result1);
    println!("{}", result2);
}
