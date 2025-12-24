package main

import "fmt"

func main() {
	// Create a Cheese Pizza
	cheesePizza := &CheesePizza{}
	cheesePizza.iPizza = cheesePizza // Link the implementation back to the template

	fmt.Println("--- Making Cheese Pizza ---")
	cheesePizza.MakePizza()

	fmt.Println("\n--- Making Pepperoni Pizza ---")
	// Create a Pepperoni Pizza
	pepperoniPizza := &PepperoniPizza{}
	pepperoniPizza.iPizza = pepperoniPizza
	pepperoniPizza.MakePizza()
}
