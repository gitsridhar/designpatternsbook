package main

// Dish represents the Originator with an internal state
type Dish struct {
	Name string
}

// DishMemento is the concrete Memento storing the Dish state
type DishMemento struct {
	name string
}

func (d *Dish) Save() *DishMemento {
	return &DishMemento{name: d.Name}
}

func (d *Dish) Restore(m *DishMemento) {
	d.Name = m.name
}
