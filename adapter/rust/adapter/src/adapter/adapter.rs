
use crate::adaptee::adaptee::Chopper;
use crate::target::target::Processor;

pub struct NewFoodProcessor {
    adaptee: Chopper,
}

impl NewFoodProcessor {
    pub fn new(adaptee: Chopper) -> Self {
        Self { adaptee }
    }
}

impl Processor for NewFoodProcessor {
    fn process_food(&self) -> String {
        return self.adaptee.chop()
    }
}