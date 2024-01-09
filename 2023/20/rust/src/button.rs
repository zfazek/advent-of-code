use crate::{moduler::Moduler, Level, Pulse, Solution, Type};

pub(crate) struct Button {
    pub name: String,
    typ: Type,
    dest: Vec<String>,
}

impl Moduler for Button {
    fn process(&mut self, _pulse: Pulse, my: &mut Solution) {
        for d in self.dest.iter() {
            let pulse = Pulse {
                from: self.name.clone(),
                level: Level::Low,
                to: d.clone(),
            };
            my.queue.push_back(pulse);
        }
    }

    fn add(&mut self, _name: String) {
    }

    fn get_type(&self) -> Type {
        self.typ
    }
}

impl Button {
    pub fn new(name: String, typ: Type, dest: Vec<String>) -> Self {
        Button { name, typ, dest }
    }
}
