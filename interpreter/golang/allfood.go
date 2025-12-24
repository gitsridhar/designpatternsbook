package main

// AllFood is a Non-Terminal Expression (AND logic)
type AllFood struct {
	Items []Item
}

func (af *AllFood) Interpret(order *FoodOrder) bool {
	for _, item := range af.Items {
		if !item.Interpret(order) {
			return false
		}
	}
	return true
}
