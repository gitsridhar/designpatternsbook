// peel.rs
use crate::customer::{Action, CustomerInteraction};

pub struct Peel {
    pub interaction: CustomerInteraction,
}

impl Action for Peel {
    fn execute(&self) {
        println!(
            "The waiter is peeling an orange for {}.",
            self.interaction.customer.name
        );
    }
}
