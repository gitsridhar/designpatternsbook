import Foundation

// 1. Action (Command) Interface
protocol Action {
    func doit()
}

// 2. Receiver Class
class Customer {
    let name: String
    
    init(name: String) {
        self.name = name
    }
    
    func orderFood() {
        print("\(name) is ordering food.")
    }
    
    func makePayment() {
        print("\(name) is making a payment.")
    }
}

// 3. Concrete Command for Customer Interaction
class CustomerInteraction: Action {
    private let customer: Customer
    private let interactionType: String
    
    init(customer: Customer, type: String) {
        self.customer = customer
        self.interactionType = type
    }
    
    func doit() {
        if interactionType == "order" {
            customer.orderFood()
        } else if interactionType == "pay" {
            customer.makePayment()
        }
    }
}

// 4. Concrete Command for Peeling
class PeelAction: Action {
    let item: String
    
    init(item: String) {
        self.item = item
    }
    
    func doit() {
        print("Peeling the \(item).")
    }
}

// 5. Invoker Class
class Waiter {
    private var actions: [Action] = []
    
    func addAction(_ action: Action) {
        actions.append(action)
    }
    
    func executeActions() {
        print("Waiter is processing the queue...")
        actions.forEach { $0.doit() }
        actions.removeAll()
    }
}

// 6. Main Implementation
let client = Customer(name: "John Doe")

// Initialize specific actions
let orderAction = CustomerInteraction(customer: client, type: "order")
let payAction = CustomerInteraction(customer: client, type: "pay")
let peelingPotato = PeelAction(item: "Potato")

// Set up the Waiter (Invoker)
let waiter = Waiter()
waiter.addAction(orderAction)
waiter.addAction(peelingPotato)
waiter.addAction(payAction)

// Execute all commands
waiter.executeActions()
