use crate::{Solution, Pulse, Type};

pub(crate) trait Moduler {
    fn process(&mut self, pulse: Pulse, my: &mut Solution);
    fn get_type(&self) -> Type;
    fn add(&mut self, name: String);
}
