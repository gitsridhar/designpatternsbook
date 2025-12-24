package main

import "fmt"

func main() {
	// Initialize components
	dish := &Dish{Name: "Pasta"}
	waiter := &Waiter{ActiveTable: 5}
	chef := &Chef{Dish: dish, Waiter: waiter}

	fmt.Printf("Initial: Dish=%s, Table=%d\n", dish.Name, waiter.ActiveTable)

	// Save initial state
	chef.Backup()

	// Modify state
	dish.Name = "Steak"
	waiter.ActiveTable = 12
	fmt.Printf("Changed: Dish=%s, Table=%d\n", dish.Name, waiter.ActiveTable)

	// Undo back to previous state
	chef.Undo()
	fmt.Printf("Restored: Dish=%s, Table=%d\n", dish.Name, waiter.ActiveTable)
}
