use crate::food::Food;

pub struct Strawberry;

impl Strawberry {
    pub fn new() -> Self {
        Strawberry {}
    }
}

impl Food for Strawberry {
    fn dip(&self) -> &str {
        "Strawberry with extra sauce"
    }
}
