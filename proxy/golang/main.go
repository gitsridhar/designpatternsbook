package main

func main() {
	vegBurger := &VegBurger{name: "Paneer Tikka Burger", price: 5.99}
	nonVegBurger := &NonVegBurger{name: "Chicken Burger", price: 7.99}

	var burger Burger

	burger = vegBurger
	println(burger.ServeBurger())

	burger = nonVegBurger
	println(burger.ServeBurger())

	// Using Proxy
	vegBurgerProxy := NewBurgerProxy("Veggie Delight", 6.49, true)
	nonVegBurgerProxy := NewBurgerProxy("Chicken Blast", 8.49, false)

	println(vegBurgerProxy.ServeBurger())
	println(nonVegBurgerProxy.ServeBurger())
}
