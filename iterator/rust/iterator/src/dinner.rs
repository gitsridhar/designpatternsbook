use crate::eating::{Dish, Eating};
use crate::restaurant::RestaurantEating;

pub trait Dinner {
    fn create_dinner(&self) -> Box<dyn Eating>;
}

pub struct WeekendDinner;

impl Dinner for WeekendDinner {
    fn create_dinner(&self) -> Box<dyn Eating> {
        let menu = vec![
            Dish { name: "Rice".to_string() },
            Dish { name: "Red Wine".to_string() },
        ];
        Box::new(RestaurantEating::new(menu))
    }
}
