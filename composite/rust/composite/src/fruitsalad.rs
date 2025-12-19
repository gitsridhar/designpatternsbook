use crate::dish::Dish;

pub struct FruitSalad {
    name: String,
}

impl FruitSalad {
    pub fn new(name: impl Into<String>) -> Self {
        FruitSalad { name: name.into() }
    }
}

impl Dish for FruitSalad {
    fn name(&self) -> &str {
        &self.name
    }

    fn operation(&self, indent: usize) -> String {
        format!("{}- FruitSalad: {}\n", " ".repeat(indent), self.name)
    }
}
