package main

func main() {
	// Initialize Chefs
	master := &MasterChef{}
	fryer := &FryingChef{}
	boiler := &BoilingChef{}
	collector := &CollectingIngredientsChef{}

	// Set up the Chain: Collector -> Boiler -> Fryer -> Master
	collector.SetNext(boiler)
	boiler.SetNext(fryer)
	fryer.SetNext(master)

	// Create a request
	order := &Dish{Name: "Signature Crispy Pasta"}

	// Start the chain
	collector.Execute(order)
}
