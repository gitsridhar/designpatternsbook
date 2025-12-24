use crate::chef::Chef;
use crate::basic_chef::BasicChef;

pub struct BoilingChef {
    pub base: BasicChef,
}

impl Chef for BoilingChef {
    fn set_next(&mut self, next: Box<dyn Chef>) { self.base.next = Some(next); }
    fn cook(&self, dish: &str) {
        if dish.contains("Soup") {
            println!("Boiling the soup...");
        }
        if let Some(ref next) = self.base.next { next.cook(dish); }
    }
}
