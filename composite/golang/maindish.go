package main

type MainDish struct {
	Dish
}

func (md *MainDish) Prepare() string {
	return "Preparing a hearty main dish."
}
