fn main() {
    let input = include_str!("../../input.txt");
    let mut vv = input
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    vv[1] = 12;
    vv[2] = 2;
    let mut n: usize = 0;
    while n < vv.len() {
        let num = vv[n];
        if num == 1 {
            let i = vv[n + 3];
            vv[i] = vv[vv[n + 1]] + vv[vv[n + 2]];
            n += 3;
            continue;
        } else if num == 2 {
            let i = vv[n + 3];
            vv[i] = vv[vv[n + 1]] * vv[vv[n + 2]];
            n += 3;
            continue;
        } else if num == 99 {
            break;
        }
        n += 1;
    }
    println!("{}", vv[0]);
}
