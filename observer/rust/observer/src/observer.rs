// observer.rs
pub trait Observer {
    fn update(&self, dish_name: &str);
}
