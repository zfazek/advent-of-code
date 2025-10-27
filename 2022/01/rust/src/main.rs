fn main() {
    main1();
    main2();
}

fn main1() {
    let mut sums: Vec<i32> = std::fs::read_to_string("../01.txt")
        .unwrap()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .filter_map(|line| line.parse::<i32>().ok())
                .sum()
        })
        .collect();
    sums.sort_unstable_by(|a, b| b.cmp(a));
    let ans2: i32 = sums.iter().take(3).sum();
    println!("{}", sums[0]);
    println!("{}", ans2);
}

fn main2() {
    // We only need to find the top 3, so we can avoid collecting all
    // sums into a Vec and sorting it.
    // We fold the iterator into a [i32; 3] array.
    let top_three = std::fs::read_to_string("../01.txt")
        .unwrap()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .filter_map(|line| line.parse::<i32>().ok())
                .sum()
        })
        // Fold all group sums to find the top 3.
        // acc[0] = largest, acc[1] = 2nd, acc[2] = 3rd
        // We assume sums are positive, so starting with [0, 0, 0] is safe.
        .fold([0; 3], |mut acc, sum| {
            if sum > acc[0] {
                // New largest: shift others down
                acc[2] = acc[1];
                acc[1] = acc[0];
                acc[0] = sum;
            } else if sum > acc[1] {
                // New 2nd largest
                acc[2] = acc[1];
                acc[1] = sum;
            } else if sum > acc[2] {
                // New 3rd largest
                acc[2] = sum;
            }
            acc
        });

    // Part 1: The largest sum
    println!("{}", top_three[0]);

    // Part 2: The sum of the top three
    let ans2: i32 = top_three.iter().sum();
    println!("{}", ans2);
}
