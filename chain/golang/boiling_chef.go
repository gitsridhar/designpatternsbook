package main

import "fmt"

type BoilingChef struct {
	next Chef
}

func (b *BoilingChef) Execute(d *Dish) {
	fmt.Printf("Boiling %s...\n", d.Name)
	d.IsBoiled = true
	if b.next != nil {
		b.next.Execute(d)
	}
}

func (b *BoilingChef) SetNext(next Chef) {
	b.next = next
}
