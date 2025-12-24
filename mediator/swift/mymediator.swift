import Foundation

// 1. Mediator Protocol
protocol Mediator {
    func notify(sender: AnyObject, message: String)
}

// 2. Base Chef Class
class Chef {
    var waiter: Mediator?
    
    func setMediator(mediator: Mediator) {
        self.waiter = mediator
    }
    
    func prepare(order: String) {
        print("Chef is working on: \(order)")
    }
}

// 3. Concrete Chef Extensions
class SoupChef: Chef {
    override func prepare(order: String) {
        print("SoupChef: Simmering the \(order)...")
        waiter?.notify(sender: self, message: "\(order) is ready!")
    }
}

class SandwichChef: Chef {
    override func prepare(order: String) {
        print("SandwichChef: Assembling the \(order)...")
        waiter?.notify(sender: self, message: "\(order) is toasted and ready!")
    }
}

// 4. Concrete Mediator (The Waiter)
class OurWaiter: Mediator {
    private var soupChef: SoupChef
    private var sandwichChef: SandwichChef
    
    init(soupChef: SoupChef, sandwichChef: SandwichChef) {
        self.soupChef = soupChef
        self.sandwichChef = sandwichChef
        
        // Embed waiter in chefs
        self.soupChef.setMediator(mediator: self)
        self.sandwichChef.setMediator(mediator: self)
    }
    
    // informChef with chef and message as parameters
    func informChef(chef: Chef, message: String) {
        print("Waiter: Passing order to chef: \(message)")
        chef.prepare(order: message)
    }
    
    func notify(sender: AnyObject, message: String) {
        if sender is SoupChef {
            print("Waiter (Notification): Serving Soup: \(message)")
        } else if sender is SandwichChef {
            print("Waiter (Notification): Serving Sandwich: \(message)")
        }
    }
}

// 5. Main Execution
let sChef = SoupChef()
let swChef = SandwichChef()
let waiter = OurWaiter(soupChef: sChef, sandwichChef: swChef)

// Interactions
waiter.informChef(chef: sChef, message: "Tomato Basil")
print("---")
waiter.informChef(chef: swChef, message: "Club Sandwich")
