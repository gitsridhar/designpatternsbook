#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

struct MySingleton {
    data: String,
}

impl MySingleton {
    fn new(data: String) -> Self {
        MySingleton { data }
    }

    fn get_data(&self) -> &str {
        &self.data
    }
}

lazy_static! {
    static ref INSTANCE: Mutex<MySingleton> = Mutex::new(MySingleton::new("Initial data".to_string()));
}

fn main() {
    let instance1 = INSTANCE.lock().unwrap();
    println!("Data from instance1: {}", instance1.get_data());

    let instance2 = INSTANCE.lock().unwrap();
    println!("Data from instance2: {}", instance2.get_data());
}