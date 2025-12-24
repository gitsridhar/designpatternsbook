package main

func main() {
	// Initialize the Mediator
	waiter := &OurWaiter{}

	// Initialize Chefs and link them to the Mediator
	soupChef := &SoupChef{
		BaseChef: BaseChef{waiter: waiter, name: "Chef Pierre"},
	}
	sandwichChef := &SandwichChef{
		BaseChef: BaseChef{waiter: waiter, name: "Chef Nicole"},
	}

	// Assign components to the Mediator
	waiter.SoupChef = soupChef
	waiter.SandwichChef = sandwichChef

	// Communication via Mediator
	waiter.InformChef(waiter.SoupChef, "One French Onion Soup")
	waiter.InformChef(waiter.SandwichChef, "One Club Sandwich")
}
