fn main() {
    let input = include_str!("../../input.txt");
    let mut vs = Vec::new();
    //let min = 7.0;
    //let max = 27.0;
    let min = 200000000000000.0;
    let max = 400000000000000.0;
    for line in input.lines() {
        let (a, b) = line.split_once(" @ ").unwrap();
        let mut coords = a.split(", ");
        let x = coords.next().unwrap().parse::<f32>().unwrap();
        let y = coords.next().unwrap().parse::<f32>().unwrap();
        let mut speeds = b.split(", ");
        let dx = speeds.next().unwrap().trim().parse::<f32>().unwrap();
        let dy = speeds.next().unwrap().trim().parse::<f32>().unwrap();
        vs.push((x, y, dx, dy));
    }
    let mut ans1: usize = 0;
    for (i, (x1, y1, dx1, dy1)) in vs.iter().enumerate() {
        for (x2, y2, dx2, dy2) in vs.iter().skip(i + 1) {
            let d1 = dy1 / dx1;
            let d2 = dy2 / dx2;
            let x = (y2 - y1 + d1 * x1 - d2 * x2) / (d1 - d2);
            let y = y1 + (x - x1) * d1;
            let future = (x >= *x1 && *dx1 >= 0.0 || x <= *x1 && *dx1 <= 0.0)
                && (x >= *x2 && *dx2 >= 0.0 || x <= *x2 && *dx2 <= 0.0);
            if x >= min && x <= max && y >= min && y <= max && future {
                ans1 += 1;
            }
        }
    }
    println!("{}", ans1);
}
