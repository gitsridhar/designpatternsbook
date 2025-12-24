use crate::chef::Chef;
use crate::basic_chef::BasicChef;

pub struct CollectingIngredientsChef {
    pub base: BasicChef,
}

impl Chef for CollectingIngredientsChef {
    fn set_next(&mut self, next: Box<dyn Chef>) { self.base.next = Some(next); }
    fn cook(&self, dish: &str) {
        println!("Collecting ingredients for: {}", dish);
        if let Some(ref next) = self.base.next { next.cook(dish); }
    }
}
