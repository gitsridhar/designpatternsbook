use crate::dish::Dish;

pub struct SaltAndPepper {
    name: String,
}

impl SaltAndPepper {
    pub fn new(name: impl Into<String>) -> Self {
        SaltAndPepper { name: name.into() }
    }
}

impl Dish for SaltAndPepper {
    fn name(&self) -> &str {
        &self.name
    }

    fn operation(&self, indent: usize) -> String {
        format!("{}- SaltAndPepper: {}\n", " ".repeat(indent), self.name)
    }
}