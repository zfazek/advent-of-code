fn main() {
    let mut n: i64 = 0;
    let key = "iwrupvqb";
    let mut first = false;
    loop {
        let plaintext = format!("{}{}", key, n);
        let s = md5::compute(&plaintext);
        let str = format!("{:x}", s);
        if !first && str.starts_with("00000") {
            println!("{}", n);
            first = true;
        }
        if str.starts_with("000000") {
            println!("{}", n);
            break;
        }
        n += 1;
    }
}
