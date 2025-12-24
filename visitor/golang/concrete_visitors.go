package main

import "fmt"

// Visitor1 implements a specific sequence of operations.
type Visitor1 struct{}

func (v *Visitor1) Drink(r Restaurant) {
	fmt.Println("Visitor1 visiting...")
	r.ServeDrink()
	r.TakePayment()
}

// Visitor2 implements a different interaction logic.
type Visitor2 struct{}

func (v *Visitor2) Drink(r Restaurant) {
	fmt.Println("Visitor2 visiting...")
	r.ServeDrink()
	// Visitor2 might choose not to trigger TakePayment or handle it differently
}
