import Foundation

// 1. Element Protocol
protocol Dish {
    var name: String { get }
}

struct FoodDish: Dish {
    let name: String
}

// 2. Iterator Protocol (Eating)
protocol Eating {
    func eat(dish: Dish)
}

// 3. Concrete Iterator (RestaurantEating)
class RestaurantEating: Eating {
    func eat(dish: Dish) {
        print("Enjoying \(dish.name) at the restaurant.")
    }
}

// 4. Abstract Factory/Creator (Dinner)
protocol Dinner {
    func createDinner() -> Eating
}

// 5. Concrete Collection (WeekendDinner)
class WeekendDinner: Dinner {
    private var dishes: [Dish] = []
    
    func add(dish: Dish) {
        dishes.append(dish)
    }
    
    func createDinner() -> Eating {
        return RestaurantEating()
    }
    
    // Implementation of the iteration logic
    func serveDinner() {
        let eatingStyle = createDinner()
        for dish in dishes {
            eatingStyle.eat(dish: dish)
        }
    }
}

// --- Main Execution ---

let myWeekendDinner = WeekendDinner()
myWeekendDinner.add(dish: FoodDish(name: "Steak Tartare"))
myWeekendDinner.add(dish: FoodDish(name: "Lobster Thermidor"))
myWeekendDinner.add(dish: FoodDish(name: "Chocolate Soufflé"))

print("Starting Weekend Dinner:")
myWeekendDinner.serveDinner()
