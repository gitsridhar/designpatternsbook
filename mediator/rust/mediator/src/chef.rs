use crate::waiter::Waiter;

/// The Component trait
pub trait Chef {
    fn receive_order(&self, order: &str);
}

/// Base Chef that holds a reference to a Waiter
pub struct BaseChef<'a> {
    pub waiter: &'a dyn Waiter,
}

/// Concrete implementation: SoupChef
pub struct SoupChef<'a> {
    pub base: BaseChef<'a>,
}

impl<'a> Chef for SoupChef<'a> {
    fn receive_order(&self, order: &str) {
        println!("Soup Chef preparing: {}", order);
    }
}

/// Concrete implementation: SandwichChef
pub struct SandwichChef<'a> {
    pub base: BaseChef<'a>,
}

impl<'a> Chef for SandwichChef<'a> {
    fn receive_order(&self, order: &str) {
        println!("Sandwich Chef preparing: {}", order);
    }
}
