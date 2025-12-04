package main

type Dish struct {
	parentDish *Dish
}

func (d *Dish) SetParent(parent *Dish) {
	d.parentDish = parent
}

func (d *Dish) GetParent() *Dish {
	return d.parentDish
}

func (d *Dish) AddDish(dish *Dish) {
	// Default implementation: do nothing
}

func (d *Dish) RemoveDish(dish *Dish) {
	// Default implementation: do nothing
}

func (d *Dish) IsComposite() bool {
	return false
}

func (d *Dish) Prepare() string {
	return "Preparing a simple dish."
}
