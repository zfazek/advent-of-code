fn main() {
    let input = include_str!("../../input.txt");
    let input = input.replace("S", "|");
    let mut vv: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut result1 = 0;
    for _ in 0..vv.len() {
        for i in (0..vv.len() - 1).rev() {
            for j in 0..vv[i].len() {
                if vv[i][j] == '|' {
                    if vv[i + 1][j] == '^' {
                        let mut split = false;
                        if j > 0 && vv[i + 1][j - 1] != '|' {
                            vv[i + 1][j - 1] = '|';
                            split = true;
                        }
                        if j < vv[i].len() - 1 && vv[i + 1][j + 1] != '|' {
                            vv[i + 1][j + 1] = '|';
                            split = true;
                        }
                        if split {
                            result1 += 1;
                        }
                    } else {
                        vv[i + 1][j] = '|';
                    }
                }
            }
        }
    }
    println!("{}", result1);

    let vv1: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut vv: Vec<Vec<i64>> = Vec::new();
    (0..vv1.len()).for_each(|i| {
        let mut v = Vec::new();
        (0..vv1[i].len()).for_each(|j| {
            let c = vv1[i][j];
            if c == '|' {
                v.push(1);
            } else if c == '^' {
                v.push(-1);
            } else {
                v.push(0);
            }
        });
        vv.push(v);
    });
    for i in 0..vv.len() - 1 {
        for j in 0..vv[i].len() {
            if vv[i][j] > 0 {
                if vv[i + 1][j] == -1 {
                    if j > 0 {
                        vv[i + 1][j - 1] += vv[i][j];
                    }
                    if j < vv[0].len() - 1 {
                        vv[i + 1][j + 1] += vv[i][j];
                    }
                } else {
                    vv[i + 1][j] += vv[i][j];
                }
            }
        }
    }
    let result3: i64 = vv[vv.len() - 1].iter().sum();
    println!("{}", result3);
}
