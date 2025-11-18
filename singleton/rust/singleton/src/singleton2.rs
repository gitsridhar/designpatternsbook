use std::sync::{LazyLock, Mutex};

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

static INSTANCE: LazyLock<Mutex<MySingleton>> = LazyLock::new(|| {
    Mutex::new(MySingleton::new("Initial data".to_string()))
});

fn main() {
    let instance1 = INSTANCE.lock().unwrap();
    println!("Data from instance1: {}", instance1.get_data());

    // Subsequent access will use the same instance
    let instance2 = INSTANCE.lock().unwrap();
    println!("Data from instance2: {}", instance2.get_data());
}