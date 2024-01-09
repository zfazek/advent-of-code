use crate::{moduler::Moduler, Level, Pulse, Solution, Type};

#[derive(PartialEq)]
enum State {
    Off,
    On,
}
pub(crate) struct FlipFlop {
    name: String,
    typ: Type,
    state: State,
    dest: Vec<String>,
}

impl Moduler for FlipFlop {
    fn process(&mut self, pulse: Pulse, my: &mut Solution) {
        if pulse.level == Level::High {
            return;
        }
        let mut level = Level::High;
        if self.state == State::Off {
            self.state = State::On;
        } else {
            self.state = State::Off;
            level = Level::Low;
        }
        for d in self.dest.iter() {
            let p = Pulse {
                from: self.name.clone(),
                level,
                to: d.clone(),
            };
            my.queue.push_back(p);
        }
    }

    fn add(&mut self, _name: String) {
    }

    fn get_type(&self) -> Type {
        self.typ
    }
}

impl FlipFlop {
    pub fn new(name: String, typ: Type, dest: Vec<String>) -> Self {
        FlipFlop {
            name,
            typ,
            state: State::Off,
            dest,
        }
    }
}
