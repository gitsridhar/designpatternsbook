package main

import "fmt"

// Waiter is a concrete observer.
type Waiter struct {
	Name string
}

func (w *Waiter) Update(orderName string) {
	fmt.Printf("Waiter %s: Picking up %s to serve to table.\n", w.Name, orderName)
}
