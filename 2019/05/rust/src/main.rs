fn main() {
    let input = include_str!("../../input.txt");
    let mut vv: Vec<i64> = input
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();
    vv[1] = 12;
    vv[2] = 2;
    let mut n: usize = 0;
    while n < vv.len() {
        let num = vv[n];
        if num == 1 {
            let i = vv[n + 3] as usize;
            vv[i] = vv[vv[n + 1] as usize] + vv[vv[n + 2] as usize];
            n += 3;
            continue;
        } else if num == 2 {
            let i = vv[n + 3] as usize;
            vv[i] = vv[vv[n + 1] as usize] * vv[vv[n + 2] as usize];
            n += 3;
            continue;
        } else if num == 3 {
            let i = vv[n + 1] as usize;
            vv[i] = vv[vv[n + 2] as usize];
            n += 2;
            continue;
        } else if num == 4 {
            let i = vv[n + 1] as usize;
            print!("{}", vv[i]);
        } else if num == 99 {
            break;
        }
        n += 1;
    }
    println!("{}", vv[0]);
}
