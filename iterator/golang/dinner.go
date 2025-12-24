package main

// Dinner is the Aggregate interface
type Dinner interface {
	CreateDinner() Eating
}

// WeekendDinner is the concrete aggregate
type WeekendDinner struct {
	dishes []*Dish
}

func (w *WeekendDinner) CreateDinner() Eating {
	return &RestaurantEating{
		dishes: w.dishes,
	}
}

func (w *WeekendDinner) AddDish(name string) {
	w.dishes = append(w.dishes, &Dish{Name: name})
}
