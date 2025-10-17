fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (rows, cols) = (grid.len(), grid[0].len());
    let (mut result1, mut result2) = (0, 0);
    for i in 0..rows {
        for j in 0..cols {
            // Part 1: Find XMAS in all 8 directions
            for (di, dj) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                if matches_word(&grid, i, j, di, dj, "XMAS") {
                    result1 += 1;
                }
            }

            // Part 2: Find X-MAS pattern (A at center)
            if i > 0 && i < rows - 1 && j > 0 && j < cols - 1 && grid[i][j] == 'A' {
                let d1 = (grid[i - 1][j - 1], grid[i + 1][j + 1]);
                let d2 = (grid[i - 1][j + 1], grid[i + 1][j - 1]);
                if matches!(d1, ('M', 'S') | ('S', 'M')) && matches!(d2, ('M', 'S') | ('S', 'M')) {
                    result2 += 1;
                }
            }
        }
    }
    dbg!(result1, result2);
}

fn matches_word(grid: &[Vec<char>], row: usize, col: usize, dr: i32, dc: i32, word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();
    for (i, &ch) in chars.iter().enumerate() {
        let r = row as i32 + dr * i as i32;
        let c = col as i32 + dc * i as i32;
        if r < 0 || r >= grid.len() as i32 || c < 0 || c >= grid[0].len() as i32 {
            return false;
        }
        if grid[r as usize][c as usize] != ch {
            return false;
        }
    }
    true
}
