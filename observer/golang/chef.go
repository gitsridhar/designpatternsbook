package main

import "fmt"

// Chef is a concrete subject that notifies waiters when food is ready.
type Chef struct {
	BaseSubject
}

func (c *Chef) CompleteOrder(orderName string) {
	fmt.Printf("Chef: Finished cooking %s\n", orderName)
	c.NotifyAll(orderName)
}
