package main

// Item is the Abstract Expression
type Item interface {
	Interpret(order *FoodOrder) bool
}

// FoodItem is a Terminal Expression
type FoodItem struct {
	FoodName string
	FoodType string
}

func (fi *FoodItem) Interpret(order *FoodOrder) bool {
	val, ok := order.Items[fi.FoodName]
	return ok && val == fi.FoodType
}

// DrinkItem is another Terminal Expression
type DrinkItem struct {
	DrinkName string
}

func (di *DrinkItem) Interpret(order *FoodOrder) bool {
	_, ok := order.Items[di.DrinkName]
	// Assumes drinks are stored with type "Drink"
	return ok && order.Items[di.DrinkName] == "Drink"
}
