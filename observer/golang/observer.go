package main

// Observer defines the interface for receiving updates from a subject.
type Observer interface {
	Update(orderName string)
}
