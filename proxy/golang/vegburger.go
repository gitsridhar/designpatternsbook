package main

import "fmt"

type VegBurger struct {
	name  string
	price float64
}

func (v *VegBurger) ServeBurger() string {
	return "Serving Veg Burger: " + v.name + " Price: " + fmt.Sprintf("%.2f", v.price)
}
