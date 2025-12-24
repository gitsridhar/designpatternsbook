package main

import "fmt"

// Eating is the Iterator interface
type Eating interface {
	HasNextDish() bool
	NextDish() *Dish
	Eat()
}

// RestaurantEating is the concrete iterator
type RestaurantEating struct {
	dishes []*Dish
	index  int
}

func (r *RestaurantEating) HasNextDish() bool {
	return r.index < len(r.dishes)
}

func (r *RestaurantEating) NextDish() *Dish {
	if r.HasNextDish() {
		dish := r.dishes[r.index]
		r.index++
		return dish
	}
	return nil
}

func (r *RestaurantEating) Eat() {
	dish := r.NextDish()
	if dish != nil {
		fmt.Printf("Eating: %s\n", dish.Name)
	}
}
