use crate::chef::Chef;

/// The Mediator trait
pub trait Waiter {
    fn inform_chef(&self, chef: &dyn Chef, message: &str);
}

/// Extended Waiter implementation
pub struct OurWaiter;

impl Waiter for OurWaiter {
    fn inform_chef(&self, chef: &dyn Chef, message: &str) {
        println!("Waiter delivering message: {}", message);
        chef.receive_order(message);
    }
}
