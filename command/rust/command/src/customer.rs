// customer.rs
pub struct Customer {
    pub name: String,
}

pub trait Action {
    fn execute(&self);
}

pub struct CustomerInteraction {
    pub customer: Customer,
}
