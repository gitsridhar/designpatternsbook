use crate::dish::Dish;

pub struct MainDish {
    name: String,
}

impl MainDish {
    pub fn new(name: impl Into<String>) -> Self {
        MainDish { name: name.into() }
    }
}

impl Dish for MainDish {
    fn name(&self) -> &str {
        &self.name
    }

    fn operation(&self, indent: usize) -> String {
        format!("{}- MainDish: {}\n", " ".repeat(indent), self.name)
    }
}
