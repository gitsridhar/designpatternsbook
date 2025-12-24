use crate::food::Food;

pub struct Sauce;

impl Sauce {
    pub fn new() -> Self {
        Sauce {}
    }
}

impl Food for Sauce {
    fn dip(&self) -> &str {
        "Food with extra sauce"
    }
}
