package main

// OurWaiter extends Waiter and coordinates specific chefs
type OurWaiter struct {
	SoupChef     *SoupChef
	SandwichChef *SandwichChef
}

func (w *OurWaiter) InformChef(chef Chef, message string) {
	chef.ReceiveOrder(message)
}
