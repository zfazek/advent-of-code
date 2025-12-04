fn main() {
    let vv_orig: Vec<Vec<_>> = include_str!("../../input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut vv_new = vv_orig.clone();
    let mut result1 = 0;
    calculate(&mut vv_new, &vv_orig, &mut result1);
    println!("{result1}");

    vv_new = vv_orig.clone();
    let mut vv_prev = vv_orig.clone();
    let mut result2 = 0;
    loop {
        if !calculate(&mut vv_new, &vv_prev, &mut result2) {
            break;
        }
        vv_prev = vv_new.clone();
    }
    println!("{result2}");
}

fn calculate(vv_new: &mut [Vec<char>], vv_prev: &[Vec<char>], result: &mut i32) -> bool {
    let mut found = false;
    for i in 0..vv_new.len() {
        for j in 0..vv_new[0].len() {
            if vv_prev[i][j] != '@' {
                continue;
            }
            let mut count = 0;
            for dj in -1_i32..2 {
                for di in -1_i32..2 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ii = i as i32 + di;
                    let jj = j as i32 + dj;
                    if ii < 0 || ii >= vv_new.len() as i32 || jj < 0 || jj >= vv_new[0].len() as i32
                    {
                        continue;
                    }
                    if vv_prev[ii as usize][jj as usize] == '@' {
                        count += 1;
                    }
                }
            }
            if count < 4 {
                vv_new[i][j] = '.';
                *result += 1;
                found = true;
            }
        }
    }
    found
}
