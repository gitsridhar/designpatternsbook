package main

import "fmt"

// IPizza defines the specific steps that change based on pizza type.
type IPizza interface {
	addDough()
	addSource()
	addToppings()
}

// Pizza represents the Template Method "skeleton".
type Pizza struct {
	iPizza IPizza
}

// MakePizza is the actual Template Method that defines the order of execution.
func (p *Pizza) MakePizza() {
	p.addDough()
	p.addSource()
	p.iPizza.addToppings() // This step is deferred to the specific implementation
	p.bake()
}

// These are default steps that can be shared or standard.
func (p *Pizza) addDough() {
	fmt.Println("Adding default pizza dough")
}

func (p *Pizza) addSource() {
	fmt.Println("Adding standard tomato sauce")
}

func (p *Pizza) bake() {
	fmt.Println("Baking pizza for 15 minutes")
}
