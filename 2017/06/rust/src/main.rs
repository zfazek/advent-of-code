fn main() {
    let mut input: Vec<i32> = std::fs::read_to_string("../06.txt")
        .unwrap()
        .split_whitespace()
        .map(|x| str::parse::<i32>(x).unwrap())
        .collect();
    let mut n = 0;
    let mut history = Vec::new();
    history.push(input.clone());
    loop {
        n += 1;
        let max_idx = get_max_idx(&input);
        make_move(&mut input, max_idx);
        //print_input(&input);
        let idx = is_seen_before(&history, &input);
        if idx != -1 {
            println!("{} {}", n, n - idx);
            break;
        }
        history.push(input.clone());
    }
}

fn get_max_idx(input: &Vec<i32>) -> usize {
    let mut max = 0;
    let mut idx = 0;
    for (i, v) in input.iter().enumerate() {
        if *v > max {
            max = *v;
            idx = i;
        }
    }
    return idx;
}

fn make_move(input: &mut Vec<i32>, max_idx: usize) {
    let mut idx = max_idx;
    let step = input[max_idx];
    input[idx] = 0;
    for _ in 0..step {
        idx = (idx + 1) % input.len();
        input[idx] += 1;
    }
}

fn print_input(input: &Vec<i32>) {
    for i in 0..input.len() {
        print!("{} ", input[i]);
    }
    println!();
}

fn is_seen_before(history: &Vec<Vec<i32>>, input: &Vec<i32>) -> i32 {
    for (i, vec) in history.iter().enumerate() {
        let mut same = true;
        for (j, v) in vec.iter().enumerate() {
            if &input[j] != v {
                same = false;
                break;
            }
        }
        if same {
            return i as i32;
        }
    }
    -1
}
