package main

// CustomerInteraction embeds Customer
type CustomerInteraction struct {
	Customer
}

func (ci *CustomerInteraction) CreateOrder(item string) Action {
	return &Peel{Item: item}
}
