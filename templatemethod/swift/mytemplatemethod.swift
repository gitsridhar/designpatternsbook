import Foundation

// Abstract Base Class
class Pizza {
    // The Template Method: defines the skeleton of the algorithm
    final func makePizza() {
        prepareDough()
        addSauce()
        addToppings() // Custom step
        bake()
        box()
    }

    private func prepareDough() {
        print("Kneading and stretching the dough.")
    }

    private func addSauce() {
        print("Adding organic tomato sauce.")
    }

    private func bake() {
        print("Baking for 15 minutes at 450°F.")
    }

    private func box() {
        print("Placing pizza in a recycled cardboard box.")
    }

    // Hook: Subclasses must implement this to provide specific toppings
    func addToppings() {
        fatalError("Subclasses must implement addToppings()")
    }
}

// Concrete Implementation 1
class CheesePizza: Pizza {
    override func addToppings() {
        print("Adding generous layers of Mozzarella and Parmesan.")
    }
}

// Concrete Implementation 2
class PepperoniPizza: Pizza {
    override func addToppings() {
        print("Adding spicy pepperoni slices and Italian herbs.")
    }
}

// Main Execution
print("--- Ordering a Cheese Pizza ---")
let cheesePizza = CheesePizza()
cheesePizza.makePizza()

print("\n--- Ordering a Pepperoni Pizza ---")
let pepperoniPizza = PepperoniPizza()
pepperoniPizza.makePizza()
