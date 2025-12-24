package main

import "fmt"

func main() {
	// 1. Create the Context (The Order)
	order := NewFoodOrder()
	order.Items["Burger"] = "Main"
	order.Items["Coke"] = "Drink"
	order.Items["Fries"] = "Side"

	// 2. Define the Expressions (The Rules)
	burger := &FoodItem{FoodName: "Burger", FoodType: "Main"}
	coke := &DrinkItem{DrinkName: "Coke"}
	pizza := &FoodItem{FoodName: "Pizza", FoodType: "Main"}

	// 3. Combine into a Non-Terminal Expression
	// Check if order contains both a Burger and a Coke
	mealCheck := &AllFood{
		Items: []Item{burger, coke},
	}

	// Check if order contains Burger, Coke, and Pizza
	partyCheck := &AllFood{
		Items: []Item{burger, coke, pizza},
	}

	// 4. Interpret
	fmt.Printf("Does order have Burger and Coke? %t\n", mealCheck.Interpret(order))
	// Output: true

	fmt.Printf("Does order have Burger, Coke, and Pizza? %t\n", partyCheck.Interpret(order))
	// Output: false
}
