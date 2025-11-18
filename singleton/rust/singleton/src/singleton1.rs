use std::sync::{OnceLock, Mutex};

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

static INSTANCE: OnceLock<Mutex<MySingleton>> = OnceLock::new();

fn get_singleton_instance(initial_data: String) -> &'static Mutex<MySingleton> {
    INSTANCE.get_or_init(|| Mutex::new(MySingleton::new(initial_data)))
}

fn main() {
    let instance1 = get_singleton_instance("Hello".to_string());
    {
        let mut singleton = instance1.lock().unwrap();
        println!("Data from instance1: {}", singleton.get_data());
        singleton.data = "World".to_string(); // Mutating the data
    }

    let instance2 = get_singleton_instance("Ignored".to_string()); // Initial data is ignored after first init
    let singleton2 = instance2.lock().unwrap();
    println!("Data from instance2: {}", singleton2.get_data()); // Will print "World"
}