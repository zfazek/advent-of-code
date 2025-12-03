fn main() {
    let (count1, count2) =
        include_str!("../../input.txt")
            .split(',')
            .fold((0, 0), |(acc1, acc2), token| {
                let (a, b) = token.split_once("-").unwrap();
                let (v1, v2) = (a.trim().parse::<usize>().unwrap()
                    ..=b.trim().parse::<usize>().unwrap())
                    .map(|n| {
                        (
                            get_invalid_id_value(n, &n.to_string(), 2),
                            get_invalid_id_value(n, &n.to_string(), n.to_string().len()),
                        )
                    })
                    .reduce(|(a1, a2), (x, y)| (a1 + x, a2 + y))
                    .unwrap();
                (acc1 + v1, acc2 + v2)
            });
    println!("{count1}");
    println!("{count2}");
}

fn get_invalid_id_value(n: usize, n_str: &str, max_len: usize) -> usize {
    (2..=max_len)
        .filter(|num_token| n_str.len().is_multiple_of(*num_token))
        .any(|num_token| {
            (1..num_token)
                .map(|i| &n_str[i * (n_str.len() / num_token)..(i + 1) * (n_str.len() / num_token)])
                .all(|token| token == &n_str[..n_str.len() / num_token])
        }) as usize
        * n
}
