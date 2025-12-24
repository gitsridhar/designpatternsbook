package main

// Restaurant defines the interface for elements that can accept a visitor.
type Restaurant interface {
	Accept(v Visitor)
	ServeDrink()
	TakePayment()
}
