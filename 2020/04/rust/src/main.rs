fn main() {
    let contents = std::fs::read_to_string("../04.txt").unwrap();
    let passports = contents.split("\n\n");
    let sum = passports
        .filter(|password| {
            let mut s = password.chars().filter(|&c| c == ':').count();
            if password.contains("cid") {
                s -= 1;
            }
            s == 7
        })
        .count();
    println!("{}", sum);
}
