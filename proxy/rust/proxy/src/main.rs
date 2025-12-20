mod burger;
mod vegburger;
mod proxy;

use burger::Burger;
use proxy::Proxy;
use vegburger::VegBurger;

fn main() {
    // Client code interacts with a Subject interface, unaware of whether it's a proxy or real subject
    let real: Box<dyn Burger> = Box::new(VegBurger);
    println!("Client using the real subject directly:");
    real.request();

    println!("\nClient using the proxy:");
    // The proxy can add logic before or after the real subject's request
    let proxy: Box<dyn Burger> = Box::new(Proxy::new(VegBurger));
    proxy.request();
}
