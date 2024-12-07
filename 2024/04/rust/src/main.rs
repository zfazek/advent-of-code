fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let msg = ['X', 'M', 'A', 'S'];
    let mut result1 = 0;
    let mut result2 = 0;
    let vv = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let len_i = vv.len() as i32;
    let len_j = vv[0].len() as i32;
    for i in 0..len_i {
        for j in 0..len_j {
            for di in [-1, 0, 1] {
                for dj in [-1, 0, 1] {
                    if di == 0 && dj == 0 {
                        if vv[i as usize][j as usize] != 'A' {
                            continue;
                        }
                        if i < 1 || i >= len_i - 1 || j < 1 || j >= len_j - 1 {
                            continue;
                        }
                        if vv[(i - 1) as usize][(j - 1) as usize] == 'M'
                            && vv[(i + 1) as usize][(j + 1) as usize] == 'S'
                            && vv[(i - 1) as usize][(j + 1) as usize] == 'M'
                            && vv[(i + 1) as usize][(j - 1) as usize] == 'S'
                        {
                            result2 += 1;
                            continue;
                        }
                        if vv[(i - 1) as usize][(j - 1) as usize] == 'M'
                            && vv[(i + 1) as usize][(j + 1) as usize] == 'S'
                            && vv[(i - 1) as usize][(j + 1) as usize] == 'S'
                            && vv[(i + 1) as usize][(j - 1) as usize] == 'M'
                        {
                            result2 += 1;
                            continue;
                        }
                        if vv[(i - 1) as usize][(j - 1) as usize] == 'S'
                            && vv[(i + 1) as usize][(j + 1) as usize] == 'M'
                            && vv[(i - 1) as usize][(j + 1) as usize] == 'M'
                            && vv[(i + 1) as usize][(j - 1) as usize] == 'S'
                        {
                            result2 += 1;
                            continue;
                        }
                        if vv[(i - 1) as usize][(j - 1) as usize] == 'S'
                            && vv[(i + 1) as usize][(j + 1) as usize] == 'M'
                            && vv[(i - 1) as usize][(j + 1) as usize] == 'S'
                            && vv[(i + 1) as usize][(j - 1) as usize] == 'M'
                        {
                            result2 += 1;
                            continue;
                        }
                    }
                    let mut good = true;
                    for n in 0..msg.len() as i32 {
                        let pos_i = i + di * n;
                        let pos_j = j + dj * n;
                        if pos_i < 0 || pos_i >= len_i || pos_j < 0 || pos_j >= len_j {
                            good = false;
                            break;
                        }
                        if vv[pos_i as usize][pos_j as usize] != msg[n as usize] {
                            good = false;
                            break;
                        }
                    }
                    if good {
                        result1 += 1;
                    }
                }
            }
        }
    }
    println!("{result1}");
    println!("{result2}");
}
