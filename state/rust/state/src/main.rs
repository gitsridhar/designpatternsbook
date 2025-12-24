mod order_phase;
mod phases;

use order_phase::OrderFood;
use phases::StartOrderPhase;

fn main() {
    // Initializing the context with the starting state
    let mut order = OrderFood::new(Box::new(StartOrderPhase));

    // Transition: Start -> Ready
    order.start();
    
    // Transition: Ready -> End (Delivered)
    order.deliver();
    
    // Finalization
    order.end();
}
