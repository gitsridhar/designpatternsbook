import Foundation

// Visitor Protocol
protocol Visitor {
    func visit(restaurant: Restaurant1)
    func visit(restaurant: Restaurant2)
    func drink(at restaurant: Restaurant)
}

// Element Protocol
protocol Restaurant {
    func accept(visitor: Visitor)
    func serveDrink()
    func takePayment()
}

// Concrete Visitors
class Visitor1: Visitor {
    func visit(restaurant: Restaurant1) {
        print("Visitor 1 is visiting Restaurant 1.")
        restaurant.serveDrink()
        drink(at: restaurant)
        restaurant.takePayment()
    }
    
    func visit(restaurant: Restaurant2) {
        print("Visitor 1 is visiting Restaurant 2.")
        restaurant.serveDrink()
        drink(at: restaurant)
        restaurant.takePayment()
    }
    
    func drink(at restaurant: Restaurant) {
        print("Visitor 1 is enjoying a beverage.")
    }
}

class Visitor2: Visitor {
    func visit(restaurant: Restaurant1) {
        print("Visitor 2 is visiting Restaurant 1.")
        restaurant.serveDrink()
        drink(at: restaurant)
        restaurant.takePayment()
    }
    
    func visit(restaurant: Restaurant2) {
        print("Visitor 2 is visiting Restaurant 2.")
        restaurant.serveDrink()
        drink(at: restaurant)
        restaurant.takePayment()
    }
    
    func drink(at restaurant: Restaurant) {
        print("Visitor 2 is drinking quickly.")
    }
}

// Concrete Restaurants
class Restaurant1: Restaurant {
    func accept(visitor: Visitor) {
        visitor.visit(restaurant: self)
    }
    
    func serveDrink() {
        print("Restaurant 1 serves a Craft Soda.")
    }
    
    func takePayment() {
        print("Restaurant 1 processed payment via Credit Card.")
    }
}

class Restaurant2: Restaurant {
    func accept(visitor: Visitor) {
        visitor.visit(restaurant: self)
    }
    
    func serveDrink() {
        print("Restaurant 2 serves an Iced Tea.")
    }
    
    func takePayment() {
        print("Restaurant 2 processed payment via Mobile Wallet.")
    }
}

// Main Execution
let restaurants: [Restaurant] = [Restaurant1(), Restaurant2()]
let visitors: [Visitor] = [Visitor1(), Visitor2()]

for restaurant in restaurants {
    for visitor in visitors {
        restaurant.accept(visitor: visitor)
        print("---")
    }
}
