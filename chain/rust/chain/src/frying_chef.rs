use crate::chef::Chef;
use crate::basic_chef::BasicChef;

pub struct FryingChef {
    pub base: BasicChef,
}

impl Chef for FryingChef {
    fn set_next(&mut self, next: Box<dyn Chef>) { self.base.next = Some(next); }
    fn cook(&self, dish: &str) {
        if dish.contains("Stir-fry") {
            println!("Frying the dish...");
        }
        if let Some(ref next) = self.base.next { next.cook(dish); }
    }
}
