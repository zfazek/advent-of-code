use std::{fs, usize};

fn main() {
    let input: Vec<String> = fs::read_to_string("../20.txt").unwrap().lines().map(str::to_string).collect();
    let algorithm = &input[0].as_bytes();
    let start = std::time::SystemTime::now();
    const T: usize = 50;
    let border = T;
    let size = input[2].len() + 2 * border;
    let mut arr = vec![vec![vec![0; 2]; size]; size];
    let mut arr_idx = 0;
    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            arr[i][j][arr_idx] = 0;
        }
    }
    for i in 2..input.len() {
        for (j, v) in input[i].chars().enumerate() {
            if v == '#' {
                arr[i - 2 + border][j + border][arr_idx] = 1;
            } else {
                arr[i - 2 + border][j + border][arr_idx] = 0;
            }
        }
    }
    // print(&arr);

    for _ in 0..T {
        for i in 0..arr.len() {
            for j in 0..arr[0].len() {
                let mut idx_str = String::from("");
                let mut c;
                for di in 0..=2 {
                    for dj in 0..=2 {
                        if i + di < 1 || i + di > arr.len() || j + dj < 1 || j + dj > arr[0].len() {
                            c = arr[0][0][arr_idx];
                        } else {
                            c = arr[i + di - 1][j + dj - 1][arr_idx];
                        }
                        if c == 1 {
                            idx_str = idx_str + "1";
                        } else {
                            idx_str = idx_str + "0";
                        }
                    }
                }
                let idx: usize = usize::from_str_radix(&idx_str, 2).unwrap();
                let char = algorithm[idx];
                if char == '#' as u8 {
                    arr[i][j][1 - arr_idx] = 1;
                } else {
                    arr[i][j][1 - arr_idx] = 0;
                }
            }
        }
        arr_idx = 1 - arr_idx;
        // print(&arr);
    }
    let mut n = 0;
    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            if arr[i][j][arr_idx] == 1 {
                n += 1;
            }
        }
    }
    let end = std::time::SystemTime::now();
    println!("{}", n);
    println!("{} ms", end.duration_since(start).unwrap().as_millis());
}

fn print(arr: &Vec<Vec<i32>>) {
    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            print!("{}", arr[i][j]);
        }
        println!();
    }
    println!();
}