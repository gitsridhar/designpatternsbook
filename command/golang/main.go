package main

import "fmt"

func main() {
	// Initialize entities
	interaction := CustomerInteraction{
		Customer: Customer{Name: "Alice"},
	}
	waiter := &Waiter{}

	fmt.Printf("Customer %s is placing orders.\n", interaction.Name)

	// Customer creates commands (actions)
	order1 := interaction.CreateOrder("Apple")
	order2 := interaction.CreateOrder("Potato")

	// Waiter receives the commands
	waiter.AddAction(order1)
	waiter.AddAction(order2)

	// Waiter triggers the execution
	fmt.Println("Waiter is processing orders:")
	waiter.ExecuteActions()
}
