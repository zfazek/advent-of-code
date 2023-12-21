use crate::{moduler::Moduler, Level, Pulse, Solution, Type};

pub(crate) struct Conjuction {
    name: String,
    typ: Type,
    input: Vec<(String, Level)>,
    dest: Vec<String>,
}

impl Moduler for Conjuction {
    fn process(&mut self, pulse: Pulse, my: &mut Solution) {
        self.update(pulse);
        let mut level = Level::High;
        if self.is_all_high() {
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

    fn add(&mut self, mut name: String) {
        name.remove(0);
        self.input.push((name, Level::Low));
    }

    fn get_type(&self) -> Type {
        self.typ
    }
}

impl Conjuction {
    pub fn new(name: String, typ: Type, dest: Vec<String>) -> Self {
        Conjuction {
            name,
            typ,
            input: vec![],
            dest,
        }
    }

    fn update(&mut self, pulse: Pulse) {
        for i in self.input.iter_mut() {
            if pulse.from == i.0 {
                i.1 = pulse.level;
                break;
            }
        }
    }

    fn is_all_high(&self) -> bool {
        self.input.iter().all(|x| x.1 == Level::High)
    }
}
