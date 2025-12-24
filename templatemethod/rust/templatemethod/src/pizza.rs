pub trait Pizza {
    // The Template Method: Defines the skeleton of the algorithm
    fn make_pizza(&self) {
        self.prepare_dough();
        self.add_sauce();
        self.add_toppings(); // Step to be overridden
        self.bake();
    }

    fn prepare_dough(&self) {
        println!("Preparing standard thin crust dough.");
    }

    fn add_sauce(&self) {
        println!("Adding tomato sauce.");
    }

    // This method is intended to be overridden by "subclasses"
    fn add_toppings(&self);

    fn bake(&self) {
        println!("Baking for 15 minutes at 400°F.");
    }
}
