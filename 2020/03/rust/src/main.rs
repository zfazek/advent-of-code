fn main() {
    let contents = std::fs::read_to_string("../03.txt").unwrap();
    let mut counter = 0;
    let mut idx = 0;
    for line in contents.lines() {
        if line.chars().nth(idx % line.len()).unwrap() == '#' {
            counter += 1;
        }
        idx += 3;
    }
    println!("{}", counter);
}
