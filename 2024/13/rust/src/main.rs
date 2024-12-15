#[derive(Debug)]
struct Button {
    x: i128,
    y: i128,
    cost: i128,
}

#[derive(Debug)]
struct Prize {
    x: i128,
    y: i128,
}

const N: i128 = 10000000000000;
//const N: i128 = 0;

// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    let machines = input.split("\n\n");
    let mut result1: i128 = 0;
    for machine in machines {
        let mut button_a = Button {
            x: 0,
            y: 0,
            cost: 3,
        };
        let mut button_b = Button {
            x: 0,
            y: 0,
            cost: 1,
        };
        let mut prize = Prize { x: 0, y: 0 };
        for line in machine.lines() {
            if line.starts_with("Button A") {
                let button_a_x = line
                    .split_once("X+")
                    .unwrap()
                    .1
                    .split_once(",")
                    .unwrap()
                    .0
                    .parse::<i128>()
                    .unwrap();
                let button_a_y = line.split_once("Y+").unwrap().1.parse::<i128>().unwrap();
                button_a = Button {
                    x: button_a_x,
                    y: button_a_y,
                    cost: 3,
                };
            }
            if line.starts_with("Button B") {
                let button_b_x = line
                    .split_once("X+")
                    .unwrap()
                    .1
                    .split_once(",")
                    .unwrap()
                    .0
                    .parse::<i128>()
                    .unwrap();
                let button_b_y = line.split_once("Y+").unwrap().1.parse::<i128>().unwrap();
                button_b = Button {
                    x: button_b_x,
                    y: button_b_y,
                    cost: 1,
                };
            }
            if line.starts_with("Prize") {
                let button_a_x = line
                    .split_once("X=")
                    .unwrap()
                    .1
                    .split_once(",")
                    .unwrap()
                    .0
                    .parse::<i128>()
                    .unwrap();
                let button_a_y = line.split_once("Y=").unwrap().1.parse::<i128>().unwrap();
                prize = Prize {
                    x: button_a_x + N,
                    y: button_a_y + N,
                };
            }
        }
        dbg!(&button_a, &button_b, &prize);
        print!("{} {}", prize.x, prize.y);
        let nom_b = prize.y * button_a.x - prize.x * button_a.y;
        let denom_b = button_b.y * button_a.x - button_a.y * button_b.x;
        dbg!(nom_b, denom_b, nom_b % denom_b);
        if nom_b % denom_b == 0 {
            println!(" OK");
            let b = nom_b / denom_b;
            let a = (prize.x - button_b.x * b) / button_a.x;
            //dbg!(&button_a, &button_b, &prize);
            let rx = a * button_a.x + b * button_b.x;
            let ry = a * button_a.y + b * button_b.y;
            println!("{a}, {b}: {rx} {ry}");
            if rx == prize.x || ry == prize.y {
                result1 += button_a.cost * a + button_b.cost * b;
            }
        } else {
            println!(" NOT OK");
            //dbg!(nom_b, denom_b, nom_b % denom_b);
            //dbg!(&button_a, &button_b, &prize);
        }
    }
    println!("{:?}", result1);
}
