fn main() {
    let file = std::fs::read_to_string("../04.txt").unwrap();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in file.lines() {
        let mut tokens = line.split(",");
        let first = tokens.next().unwrap();
        let mut t1 = first.split("-");
        let a1 = t1.next().unwrap().parse::<i32>().unwrap();
        let a2 = t1.next().unwrap().parse::<i32>().unwrap();
        let second = tokens.next().unwrap();
        let mut t2 = second.split("-");
        let b1 = t2.next().unwrap().parse::<i32>().unwrap();
        let b2 = t2.next().unwrap().parse::<i32>().unwrap();
        if a1 <= b2 && a2 >= b1 || b1 <= a2 && b2 >= a1 {
            sum2 += 1;
        }
        if a1 <= b1 && a2 >= b2 || b1 <= a1 && b2 >= a2{
            sum1 += 1;
        }
    }
    println!("{}", sum1);
    println!("{}", sum2);
}
