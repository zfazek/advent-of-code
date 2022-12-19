fn main() {
    let file = std::fs::read_to_string("../18.txt").unwrap();
    let mut s = std::collections::HashSet::new();
    for line in file.lines() {
        let mut tokens = line.split(",");
        let x = tokens.next().unwrap().parse::<i32>().unwrap();
        let y = tokens.next().unwrap().parse::<i32>().unwrap();
        let z = tokens.next().unwrap().parse::<i32>().unwrap();
        let n = 100;
        //println!("{} {} {}", x, y, z);
        let k = ((x-1)*n+x, y, z);
        if s.contains(&k) {
            s.remove(&k);
        } else {
            s.insert(k);
        }
        let k = (x*n+x+1, y, z);
        if s.contains(&k) {
            s.remove(&k);
        } else {
            s.insert(k);
        }
        let k = (x, (y-1)*n+y, z);
        if s.contains(&k) {
            s.remove(&k);
        } else {
            s.insert(k);
        }
        let k = (x, y*n+y+1, z);
        if s.contains(&k) {
            s.remove(&k);
        } else {
            s.insert(k);
        }
        let k = (x, y, (z-1)*n+z);
        if s.contains(&k) {
            s.remove(&k);
        } else {
            s.insert(k);
        }
        let k = (x, y, z*n+z+1);
        if s.contains(&k) {
            s.remove(&k);
        } else {
            s.insert(k);
        }
    }
    println!("{}", s.len());
}
