use crate::broadcast::Broadcast;
use crate::button::Button;
use crate::conjuction::Conjuction;
use crate::flipflop::FlipFlop;
use crate::moduler::Moduler;
use num::integer::lcm;
use numfmt::*;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::io::Write;

mod broadcast;
mod button;
mod conjuction;
mod flipflop;
mod moduler;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Level {
    Low,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Type {
    Button,
    Broadcast,
    FlipFlop,
    Conjuction,
}

#[derive(Clone, Debug)]
struct Pulse {
    from: String,
    level: Level,
    to: String,
}

struct Solution {
    conf: BTreeMap<String, Vec<String>>,
    queue: VecDeque<Pulse>,
}

impl Solution {
    fn new() -> Self {
        let conf = BTreeMap::new();
        let queue = VecDeque::new();
        Solution { conf, queue }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let mut ans2: usize = 1;
    let mut f = Formatter::default();
    let targets = ["js", "zb", "bs", "rr"];
    for target in targets {
        let mut my = Solution::new();
        let mut modules: BTreeMap<String, Box<dyn Moduler>> = BTreeMap::new();
        for line in input.lines() {
            let (module, dest) = line.split_once(" -> ").unwrap();
            let typ = module.chars().next().unwrap();
            let mut name = module.to_string();
            if typ != 'b' {
                name.remove(0);
            }
            let ms = dest.split(", ").map(|x| x.to_string()).collect::<Vec<_>>();
            my.conf.insert(name.to_owned(), ms.clone());
            match typ {
                'b' => {
                    let m: Broadcast = Broadcast::new(name.to_owned(), Type::Broadcast, ms);
                    modules.insert(name, Box::new(m));
                }
                '%' => {
                    let m: FlipFlop = FlipFlop::new(name.to_owned(), Type::FlipFlop, ms);
                    modules.insert(name, Box::new(m));
                }
                '&' => {
                    let m: Conjuction = Conjuction::new(name.to_owned(), Type::Conjuction, ms);
                    modules.insert(name, Box::new(m));
                }
                _ => {}
            }
        }
        for (module_name, value) in modules.iter_mut() {
            if value.get_type() == Type::Conjuction {
                for line in input.lines() {
                    let (module, dest) = line.split_once(" -> ").unwrap();
                    let name = module.to_string();
                    let ms = dest.split(", ").map(|x| x.to_string()).collect::<Vec<_>>();
                    if ms.contains(module_name) {
                        value.add(name);
                    }
                }
            }
        }
        let mut _n_low: usize = 0;
        let mut _n_high: usize = 0;
        'foo: for n in 1.. {
            let m: Button = Button::new(
                "button".to_string(),
                Type::Button,
                vec!["broadcaster".to_string()],
            );
            modules.insert(m.name.to_owned(), Box::new(m));
            let pulse = Pulse {
                from: String::from(""),
                level: Level::Low,
                to: String::from(""),
            };
            modules
                .get_mut(&"button".to_string())
                .unwrap()
                .process(pulse, &mut my);

            while !my.queue.is_empty() {
                let pulse = my.queue.pop_front().unwrap();
                if pulse.level == Level::Low {
                    _n_low += 1;
                } else {
                    _n_high += 1;
                }
                if let Some(m) = modules.get_mut(&pulse.to) {
                    m.process(pulse.clone(), &mut my);
                    if pulse.to == target && pulse.level == Level::Low {
                        println!("{}: {}", target, n);
                        ans2 = lcm(ans2, n);
                        break 'foo;
                    }
                }
            }
            if n % 1000000 == 0 && n > 0 {
                print!("\r{}", f.fmt2(n));
                std::io::stdout().flush().unwrap();
            }
        }
        // println!("{}", _n_low * _n_high);
    }
    println!("{}", ans2);
}
