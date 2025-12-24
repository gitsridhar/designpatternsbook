use crate::chef::Chef;
use crate::basic_chef::BasicChef;

pub struct MasterChef {
    pub base: BasicChef,
}

impl Chef for MasterChef {
    fn set_next(&mut self, next: Box<dyn Chef>) { self.base.next = Some(next); }
    fn cook(&self, dish: &str) {
        println!("Master Chef performing final garnish on {}!", dish);
        // End of chain; no call to next
    }
}
