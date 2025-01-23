fn main() {
    let lines = std::fs::read_to_string("../02.txt").unwrap();
    let mut result1 = 0;
    let mut result2 = 0;
    for line in lines.lines() {
        let mut nums = [0; 3];
        let mut tokens = line.split('x');
        let l = str::parse::<usize>(tokens.next().unwrap()).unwrap();
        let w = str::parse::<usize>(tokens.next().unwrap()).unwrap();
        let h = str::parse::<usize>(tokens.next().unwrap()).unwrap();
        nums[0] = l;
        nums[1] = w;
        nums[2] = h;
        nums.sort();
        result1 += 2 * l * w + 2 * l * h + 2 * w * h + nums[0] * nums[1];
        result2 += 2 * nums[0] + 2 * nums[1] + l * w * h;
    }
    println!("{}", result1);
    println!("{}", result2);
}
