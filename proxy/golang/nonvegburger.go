package main

import "fmt"

type NonVegBurger struct {
	name  string
	price float64
}

func (n *NonVegBurger) ServeBurger() string {
	return "Serving Non-Veg Burger: " + n.name + " Price: " + fmt.Sprintf("%.2f", n.price)
}
