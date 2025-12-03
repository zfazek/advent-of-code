fn main() {
    let (count1, count2, _) = include_str!("../../input.txt").lines().fold(
        (0, 0, 50),
        |(mut count1, mut count2, mut dial), line| {
            if let Some(c) = line.chars().next()
                && let Ok(v) = line[1..].parse::<i32>()
            {
                let sign = if c == 'L' { -1 } else { 1 };
                for _ in 0..v {
                    dial += sign;
                    if dial % 100 == 0 {
                        count2 += 1;
                    }
                }
                if dial % 100 == 0 {
                    count1 += 1;
                }
            }
            (count1, count2, dial % 100)
        },
    );
    println!("{count1}");
    println!("{count2}");
}
