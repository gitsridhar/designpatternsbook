use crate::food::Food;

pub struct ChocolateSauce<'a> {
    food: &'a dyn Food,
}

impl<'a> ChocolateSauce<'a> {
    pub fn new(food: &'a dyn Food) -> Self {
        ChocolateSauce { food }
    }
    pub fn dip(&self) -> String {
        self.food.dip().to_owned() + " with extra chocolate sauce"
    }
}