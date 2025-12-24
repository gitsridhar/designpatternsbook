package main

import "fmt"

type CollectingIngredientsChef struct {
	next Chef
}

func (c *CollectingIngredientsChef) Execute(d *Dish) {
	if d.IngredientsReady {
		fmt.Printf("Ingredients for %s already collected.\n", d.Name)
	} else {
		fmt.Printf("Collecting ingredients for %s...\n", d.Name)
		d.IngredientsReady = true
	}
	if c.next != nil {
		c.next.Execute(d)
	}
}

func (c *CollectingIngredientsChef) SetNext(next Chef) {
	c.next = next
}
