fn main() {
    one();
    two();
}

fn one() {
    let dirs: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let n = 265149;
    let mut grid = std::collections::HashSet::new();
    let mut d_idx = 0;
    let mut i = 0;
    let mut x = 0;
    let mut y = 0;
    i += 1;
    grid.insert((x, y));
    x = x + dirs[d_idx].0;
    y = y + dirs[d_idx].1;
    i += 1;
    grid.insert((x, y));
    while i < n {
        d_idx = 0;
        i += 1;
        loop {
            if !grid.contains(&(x + dirs[d_idx].0, y + dirs[d_idx].1)) {
                let next_d_idx =(d_idx + 1) % dirs.len(); 
                let next_x = x + dirs[next_d_idx].0;
                let next_y = y + dirs[next_d_idx].1;
                if grid.contains(&(next_x, next_y)) {
                    x = x + dirs[d_idx].0;
                    y = y + dirs[d_idx].1;
                    grid.insert((x, y));
                    break;
                }
            }
            d_idx = (d_idx + 1) % dirs.len();
        }
    }
    println!("i: {}, ({}, {}), manhattan: {}", i, x, y, i32::abs(x) + i32::abs(y));
}

fn two() {
    let dirs: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut grid = std::collections::HashMap::new();
    let mut d_idx = 0;
    let mut i = 0;
    let mut x = 0;
    let mut y = 0;
    i += 1;
    grid.insert((x, y), 1);
    x = x + dirs[d_idx].0;
    y = y + dirs[d_idx].1;
    i += 1;
    grid.insert((x, y), 1);
    while grid.get(&(x, y)).unwrap() < &265149 {
        d_idx = 0;
        i += 1;
        loop {
            if let None = grid.get(&(x + dirs[d_idx].0, y + dirs[d_idx].1)) {
                let next_d_idx =(d_idx + 1) % dirs.len(); 
                let next_x = x + dirs[next_d_idx].0;
                let next_y = y + dirs[next_d_idx].1;
                if let Some(_) = grid.get(&(next_x, next_y)) {
                    x = x + dirs[d_idx].0;
                    y = y + dirs[d_idx].1;
                    let value = get_neighbors(&grid, x, y);
                    grid.insert((x, y), value);
                    break;
                }
            }
            d_idx = (d_idx + 1) % dirs.len();
        }
    }
    println!("i: {}, ({}, {}), value: {}", i, x, y, grid.get(&(x, y)).unwrap());
}

fn get_neighbors(grid: &std::collections::HashMap<(i32, i32), i32>, x: i32, y: i32) -> i32 {
    let mut sum = 0;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            let x = x + i;
            let y = y + j;
            if let Some(value) = grid.get(&(x, y)) {
                sum += value;
            }
        }
    }
    sum
}
