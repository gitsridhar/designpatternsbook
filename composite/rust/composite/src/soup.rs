use crate::dish::Dish;

pub struct Soup {
    name: String,
}

impl Soup {
    pub fn new(name: impl Into<String>) -> Self {
        Soup { name: name.into() }
    }
}

impl Dish for Soup {
    fn name(&self) -> &str {
        &self.name
    }

    fn operation(&self, indent: usize) -> String {
        format!("{}- Soup: {}\n", " ".repeat(indent), self.name)
    }
}