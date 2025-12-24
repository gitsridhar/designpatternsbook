package main

import "fmt"

// Restaurant1 is a concrete element.
type Restaurant1 struct {
	Name string
}

func (r *Restaurant1) Accept(v Visitor) { v.Drink(r) }
func (r *Restaurant1) ServeDrink()      { fmt.Printf("%s is serving a soda.\n", r.Name) }
func (r *Restaurant1) TakePayment()     { fmt.Println("Payment taken via Cash at Restaurant1.") }

// Restaurant2 is another concrete element.
type Restaurant2 struct {
	Name string
}

func (r *Restaurant2) Accept(v Visitor) { v.Drink(r) }
func (r *Restaurant2) ServeDrink()      { fmt.Printf("%s is serving a juice.\n", r.Name) }
func (r *Restaurant2) TakePayment()     { fmt.Println("Payment taken via Card at Restaurant2.") }
