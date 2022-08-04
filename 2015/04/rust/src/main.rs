fn main() {
    let mut n:i64 = 0;
    let key = "iwrupvqb";
    let mut flag = false;
    loop {
        let plaintext = format!("{}{}", key, n);
        let s = md5::compute(&plaintext);
        let str = format!("{:x}", s);
        if !flag && str.starts_with("00000") {
            println!("{}", n);
            flag = true;
        }
        if str.starts_with("000000") {
            println!("{}", n);
            break;
        }
        n += 1;
    }
}
