package main

import "fmt"

type CheesePizza struct {
	Pizza
}

func (c *CheesePizza) addToppings() {
	fmt.Println("Adding extra Mozzarella and Parmesan cheese")
}
