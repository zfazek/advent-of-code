use crate::{moduler::Moduler, Pulse, Solution, Type};

pub(crate) struct Broadcast {
    name: String,
    typ: Type,
    dest: Vec<String>,
}

impl Moduler for Broadcast {
    fn process(&mut self, pulse: Pulse, my: &mut Solution) {
        for d in self.dest.iter() {
            let level = pulse.level;
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

impl Broadcast {
    pub fn new(name: String, typ: Type, dest: Vec<String>) -> Self {
        Broadcast { name, typ, dest }
    }
}
