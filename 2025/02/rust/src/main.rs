fn main() {
    let input = include_str!("../../input.txt");
    let tokens = input.split(',');
    let mut count1: usize = 0;
    let mut count2: usize = 0;
    for token in tokens {
        if let Some((a, b)) = token.split_once("-")
            && let Ok(a) = a.trim().parse::<usize>()
            && let Ok(b) = b.trim().parse::<usize>()
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

fn get_invalid_id_value(n: usize, n_str: &str, max_len: usize) -> usize {
    let len = n_str.len();
    (2..=max_len)
        .filter(|num_token| len.is_multiple_of(*num_token))
        .any(|num_token| {
            let l = len / num_token;
            let first_token = &n_str[..l];
            (1..num_token)
                .map(|i| &n_str[i * l..(i + 1) * l])
                .all(|token| token == first_token)
        }) as usize
        * n
}
