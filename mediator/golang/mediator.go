package main

// Waiter - The Mediator Interface
type Waiter interface {
	InformChef(chef Chef, message string)
}

// Chef - The Component Interface
type Chef interface {
	ReceiveOrder(message string)
	GetName() string
}

// BaseChef - Base class to be embedded
type BaseChef struct {
	waiter Waiter
	name   string
}

func (c *BaseChef) GetName() string {
	return c.name
}
