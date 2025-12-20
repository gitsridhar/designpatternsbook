
use crate::burger::Burger;
use crate::vegburger::VegBurger;

pub struct Proxy {
    real_burger: VegBurger,
}

impl Proxy {
    pub fn new(real_burger: VegBurger) -> Self {
        Proxy { real_burger }
    }

    fn tastes_good(&self) -> bool {
        println!("Proxy: Checking taste.");
        true
    }

    fn is_healthy(&self) -> bool {
        println!("Proxy: checking healthy.");
        true
    }
}

// The Proxy implements the Burger trait and delegates the core logic to the VegBurger
impl Burger for Proxy {
    fn request(&self) {
        if self.tastes_good() {
            if self.is_healthy() {
                self.real_burger.request();
            } else {
                println!("Proxy: Not Healthy!");
            }
        } else {
            println!("Proxy: Not Tasty")
        }
    }
}
