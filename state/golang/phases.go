package main

import "fmt"

// StartOrderPhase - User is browsing
type StartOrderPhase struct {
	BasePhase
	Order *OrderFood
}

func (s *StartOrderPhase) AddItem() {
	fmt.Println("Item added! Moving to Ready phase.")
	s.Order.SetState(&ReadyOrderPhase{Order: s.Order})
}

// ReadyOrderPhase - Items are in cart, ready to checkout
type ReadyOrderPhase struct {
	BasePhase
	Order *OrderFood
}

func (r *ReadyOrderPhase) AddItem() {
	fmt.Println("Another item added to cart.")
}

func (r *ReadyOrderPhase) Confirm() {
	fmt.Println("Payment successful! Moving to End phase.")
	r.Order.SetState(&EndOrderPhase{Order: r.Order})
}

// EndOrderPhase - Order is complete
type EndOrderPhase struct {
	BasePhase
	Order *OrderFood
}

func (e *EndOrderPhase) AddItem() {
	fmt.Println("Error: Cannot add items to a completed order.")
}
