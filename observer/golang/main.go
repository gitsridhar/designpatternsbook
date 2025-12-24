package main

func main() {
	// Initialize the Subject (Chef)
	chef := &Chef{}

	// Initialize Observers (Waiters)
	waiter1 := &Waiter{Name: "Alice"}
	waiter2 := &Waiter{Name: "Bob"}

	// Register observers
	chef.Register(waiter1)
	chef.Register(waiter2)

	// Trigger an update
	chef.CompleteOrder("Margherita Pizza")

	// Deregister one observer and trigger another update
	chef.Deregister(waiter1)
	chef.CompleteOrder("Spaghetti Carbonara")
}
