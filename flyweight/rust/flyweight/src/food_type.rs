
/// The Flyweight (intrinsic state)
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FoodType {
    pub name: String,
    pub description: String,
    pub price: u32,
}

impl FoodType {
    pub fn new(name: String, description: String, price: u32) -> Self {
        Self { name, description, price }
    }

    pub fn display(&self, serving_size: &str, table_number: u32) {
        println!(
            "Serving '{}' ({} - {}) for ${} to Table {}",
            self.name,
            self.description,
            serving_size, // Extrinsic state passed as parameter
            self.price,
            table_number  // Extrinsic state passed as parameter
        );
    }
}
