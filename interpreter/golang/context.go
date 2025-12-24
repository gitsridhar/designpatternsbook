package main

// FoodOrder serves as the Context
type FoodOrder struct {
	// Map of FoodName to FoodType (e.g., "Pizza": "Main")
	Items map[string]string
}

func NewFoodOrder() *FoodOrder {
	return &FoodOrder{Items: make(map[string]string)}
}
