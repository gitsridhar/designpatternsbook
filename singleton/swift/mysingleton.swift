class Singleton {
    static let shared = Singleton()
    
    private init() {
        // Private initialization to ensure just one instance is created.
    }
    
    func doSomething() {
        print("Doing something!")
    }
}
// Usage
let singletonInstance1 = Singleton.shared
let singletonInstance2 = Singleton.shared
singletonInstance1.doSomething()
print(singletonInstance1 === singletonInstance2) // true    // Confirming both instances are the same   
singletonInstance2.doSomething()
