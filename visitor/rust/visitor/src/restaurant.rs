use crate::visitor::Visitor;

pub trait Restaurant {
    fn accept(&self, visitor: &dyn Visitor);
    fn serve_drink(&self, visitor: &dyn Visitor);
    fn take_payment(&self, visitor: &dyn Visitor);
}

pub struct Restaurant1;
impl Restaurant for Restaurant1 {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_restaurant1(self);
    }
    fn serve_drink(&self, _v: &dyn Visitor) {
        println!("Restaurant 1 serves a cold soda.");
    }
    fn take_payment(&self, _v: &dyn Visitor) {
        println!("Restaurant 1 processed a cash payment.");
    }
}

pub struct Restaurant2;
impl Restaurant for Restaurant2 {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_restaurant2(self);
    }
    fn serve_drink(&self, _v: &dyn Visitor) {
        println!("Restaurant 2 serves a glass of wine.");
    }
    fn take_payment(&self, _v: &dyn Visitor) {
        println!("Restaurant 2 processed a credit card payment.");
    }
}
