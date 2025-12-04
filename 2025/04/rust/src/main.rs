fn main() {
    let grid_orig: Vec<Vec<_>> = include_str!("../../input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut grid_new = grid_orig.clone();
    let mut result1 = 0;
    calculate(&mut grid_new, &grid_orig, &mut result1);
    println!("{result1}");

    grid_new = grid_orig.clone();
    let mut grid_prev = grid_orig.clone();
    let mut result2 = 0;
    loop {
        if !calculate(&mut grid_new, &grid_prev, &mut result2) {
            break;
        }
        grid_prev = grid_new.clone();
    }
    println!("{result2}");
}

fn calculate(grid_new: &mut [Vec<char>], grid_prev: &[Vec<char>], changes: &mut i32) -> bool {
    let mut found = false;
    let len = grid_new.len();
    for i in 0..len {
        for j in 0..len {
            if grid_prev[i][j] != '@' {
                continue;
            }
            let mut count = 0;
            for dj in -1..2 {
                for di in -1..2 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let new_i = i as i32 + di;
                    let new_j = j as i32 + dj;
                    if new_i < 0 || new_i >= len as i32 || new_j < 0 || new_j >= len as i32 {
                        continue;
                    }
                    if grid_prev[new_i as usize][new_j as usize] == '@' {
                        count += 1;
                    }
                }
            }
            if count < 4 {
                grid_new[i][j] = '.';
                *changes += 1;
                found = true;
            }
        }
    }
    found
}
