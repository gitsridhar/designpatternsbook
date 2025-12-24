mod waiter;
mod chef;

use waiter::{OurWaiter, Waiter};
use chef::{BaseChef, SoupChef, SandwichChef};

fn main() {
    // Initialize the Mediator
    let mediator = OurWaiter;

    // Initialize Chefs with the Mediator embedded
    let soup_chef = SoupChef {
        base: BaseChef { waiter: &mediator },
    };
    
    let sandwich_chef = SandwichChef {
        base: BaseChef { waiter: &mediator },
    };

    // The Waiter (Mediator) coordinates the communication
    mediator.inform_chef(&soup_chef, "Tomato Basil Soup");
    mediator.inform_chef(&sandwich_chef, "Club Sandwich");
}
