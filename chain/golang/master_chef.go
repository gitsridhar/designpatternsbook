package main

import "fmt"

type MasterChef struct {
	next Chef
}

func (m *MasterChef) Execute(d *Dish) {
	fmt.Printf("Master Chef is plating and garnishing %s.\n", d.Name)
	d.IsPlated = true
	fmt.Printf("Order for %s is COMPLETE!\n", d.Name)
}

func (m *MasterChef) SetNext(next Chef) {
	m.next = next
}
