package main

import "fmt"

type PepperoniPizza struct {
	Pizza
}

func (p *PepperoniPizza) addToppings() {
	fmt.Println("Adding spicy pepperoni slices and oregano")
}
