package main

import "fmt"

func main() {
	// Initialize context
	order := &OrderFood{}

	// Set initial state
	initialState := &StartOrderPhase{Order: order}
	order.SetState(initialState)

	fmt.Println("--- New Order Process ---")

	// State: Start -> Ready
	order.AddItem()

	// State: Ready
	order.AddItem()
	order.Confirm()

	// State: End
	order.AddItem()
}
