use crate::restaurant::Restaurant;

pub trait Visitor {
    fn visit_restaurant1(&self, restaurant: &dyn Restaurant);
    fn visit_restaurant2(&self, restaurant: &dyn Restaurant);
    
    // The specific "drink" method requested
    fn drink(&self, restaurant: &dyn Restaurant) where Self: Sized {
        println!("Visitor is preparing to drink...");
        restaurant.serve_drink(self);
    }
}

pub struct Visitor1;
impl Visitor for Visitor1 {
    fn visit_restaurant1(&self, _r: &dyn Restaurant) {
        println!("Visitor 1 is visiting Restaurant 1 (Casual Dining).");
    }
    fn visit_restaurant2(&self, _r: &dyn Restaurant) {
        println!("Visitor 1 is visiting Restaurant 2 (Fine Dining).");
    }
}

pub struct Visitor2;
impl Visitor for Visitor2 {
    fn visit_restaurant1(&self, _r: &dyn Restaurant) {
        println!("Visitor 2 is visiting Restaurant 1 (Fast Food).");
    }
    fn visit_restaurant2(&self, _r: &dyn Restaurant) {
        println!("Visitor 2 is visiting Restaurant 2 (Cafe).");
    }
}
