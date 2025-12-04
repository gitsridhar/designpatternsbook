package main

type FruitSalad struct {
	Dish
}

func (fs *FruitSalad) Prepare() string {
	return "Preparing a delicious fruit salad."
}
