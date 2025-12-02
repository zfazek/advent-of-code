fn main() {
    let input = include_str!("../../input.txt");
    let tokens = input.split(',');
    let mut count1: i64 = 0;
    let mut count2: i64 = 0;
    for token in tokens {
        if let Some((a, b)) = token.split_once("-")
            && let Ok(a) = a.trim().parse::<i64>()
            && let Ok(b) = b.trim().parse::<i64>()
        {
            for n in a..=b {
                let n_str = n.to_string();
                count1 += get_invalid_id_value(n, &n_str, 2);
                count2 += get_invalid_id_value(n, &n_str, n_str.len());
            }
        }
    }
    println!("{count1}");
    println!("{count2}");
}

fn get_invalid_id_value(n: i64, n_str: &str, max_len: usize) -> i64 {
    let len = n_str.len();
    for num_token in 2..=max_len {
        if !len.is_multiple_of(num_token) {
            continue;
        }
        let l = len / num_token;
        let first_token = &n_str[..l];
        let found = (1..num_token)
            .map(|i| &n_str[i * l..(i + 1) * l])
            .all(|token| token == first_token);
        if found {
            return n;
        }
    }
    0
}
