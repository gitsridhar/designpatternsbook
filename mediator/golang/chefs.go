package main

import "fmt"

// SoupChef extends BaseChef
type SoupChef struct {
	BaseChef
}

func (s *SoupChef) ReceiveOrder(message string) {
	fmt.Printf("[Soup Station] %s received order: %s\n", s.name, message)
}

// SandwichChef extends BaseChef
type SandwichChef struct {
	BaseChef
}

func (s *SandwichChef) ReceiveOrder(message string) {
	fmt.Printf("[Sandwich Station] %s received order: %s\n", s.name, message)
}
