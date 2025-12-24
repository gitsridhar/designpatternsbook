package main

// Visitor defines the abstract operations for visiting elements.
type Visitor interface {
	Drink(r Restaurant)
}
