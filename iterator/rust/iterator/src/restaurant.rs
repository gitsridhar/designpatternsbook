use crate::eating::{Dish, Eating};

pub struct RestaurantEating {
    dishes: Vec<Dish>,
}

impl RestaurantEating {
    pub fn new(dishes: Vec<Dish>) -> Self {
        Self { dishes }
    }
}

impl Eating for RestaurantEating {
    fn get_dishes(&self) -> Box<dyn Iterator<Item = &Dish> + '_> {
        // Using the vector's built-in iterator
        Box::new(self.dishes.iter())
    }
}
