import Foundation

// Context: Holds the state of the order
class FoodOrder {
    var items: [String: String] = [:] // Map of FoodItem Name to FoodType
    
    func add(item: String, type: String) {
        items[item] = type
    }
}

// Abstract Expression
protocol Item {
    func interpret(order: FoodOrder) -> String
}

// Terminal Expression: Food
class FoodItem: Item {
    private var name: String
    
    init(name: String) {
        self.name = name
    }
    
    func interpret(order: FoodOrder) -> String {
        if let type = order.items[name] {
            return "Food: \(name) (\(type))"
        }
        return ""
    }
}

// Terminal Expression: Drink
class DrinkItem: Item {
    private var name: String
    
    init(name: String) {
        self.name = name
    }
    
    func interpret(order: FoodOrder) -> String {
        if let type = order.items[name] {
            return "Drink: \(name) (\(type))"
        }
        return ""
    }
}

// Non-terminal Expression: Combines Food and Drink
class AllFood: Item {
    private var food: FoodItem
    private var drink: DrinkItem
    
    init(food: FoodItem, drink: DrinkItem) {
        self.food = food
        self.drink = drink
    }
    
    func interpret(order: FoodOrder) -> String {
        return "\(food.interpret(order: order)) and \(drink.interpret(order: order))"
    }
}

// --- Main Execution ---

let myOrder = FoodOrder()
myOrder.add(item: "Burger", type: "Main Course")
myOrder.add(item: "Cola", type: "Beverage")

let burger = FoodItem(name: "Burger")
let soda = DrinkItem(name: "Cola")

// Interpret individual items
print(burger.interpret(order: myOrder)) // Output: Food: Burger (Main Course)

// Interpret combined expression
let combo = AllFood(food: burger, drink: soda)
print("Combo Order: \(combo.interpret(order: myOrder))") 
// Output: Combo Order: Food: Burger (Main Course) and Drink: Cola (Beverage)
