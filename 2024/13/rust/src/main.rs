use std::collections::BTreeSet;

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
    let input = std::fs::read_to_string("../inputm.txt").unwrap();
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
        let nom_b = prize.y * button_a.x - prize.x * button_a.y;
        let denom_b = button_b.y * button_a.x - button_a.y * button_b.x;
        //dbg!(nom_b, denom_b, nom_b % denom_b);
        if nom_b % denom_b == 0 {
            let b = nom_b / denom_b;
            let a = (prize.x - button_b.x * b) / button_a.x;
            //dbg!(&button_a, &button_b, &prize);
            dbg!(a, b);
            result1 += button_a.cost * a + button_b.cost * b;
        }
        /*
        let mut sol: BTreeSet<(i128, i128)> = BTreeSet::new();
        let mut visited: BTreeSet<(i128, i128)> = BTreeSet::new();
        _iter(&button_a, &button_b, &prize, 0, 0, &mut sol, &mut visited);
        if sol.len() > 1 {
            println!("{:?}", &sol);
        }
        if let Some(min) = sol
            .iter()
            .map(|x| button_a.cost * x.0 + button_b.cost * x.1)
            .min()
        {
            result1 += min;
        }
        */
    }
    println!("{:?}", result1);
}

fn _iter(
    ba: &Button,
    bb: &Button,
    p: &Prize,
    a: i128,
    b: i128,
    sol: &mut BTreeSet<(i128, i128)>,
    visited: &mut BTreeSet<(i128, i128)>,
) {
    //println!("{} {}", a, b);
    if visited.contains(&(a, b)) {
        return;
    }
    if a * ba.x + b * bb.x > p.x || a * ba.y + b * bb.y > p.y {
        return;
    }
    visited.insert((a, b));
    if a * ba.x + b * bb.x == p.x && a * ba.y + b * bb.y == p.y {
        sol.insert((a, b));
        return;
    }
    _iter(ba, bb, p, a + 1, b, sol, visited);
    _iter(ba, bb, p, a, b + 1, sol, visited);
}
