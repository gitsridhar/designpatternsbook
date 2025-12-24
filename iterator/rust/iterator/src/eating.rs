pub struct Dish {
    pub name: String,
}

pub trait Eating {
    // Standard iterator pattern in Rust returns a type implementing Iterator
    fn get_dishes(&self) -> Box<dyn Iterator<Item = &Dish> + '_>;
}
