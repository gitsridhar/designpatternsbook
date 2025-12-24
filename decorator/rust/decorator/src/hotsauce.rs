use crate::food::Food;

pub struct HotSauce<'a> {
    food: &'a dyn Food,
}

impl<'a> HotSauce<'a> {
    pub fn new(food: &'a dyn Food) -> Self {
        HotSauce { food }
    }
    pub fn dip(&self) -> String {
        self.food.dip().to_owned() + " with extra hot sauce"
    }
}
