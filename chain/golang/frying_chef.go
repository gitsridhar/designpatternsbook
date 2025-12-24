package main

import "fmt"

type FryingChef struct {
	next Chef
}

func (f *FryingChef) Execute(d *Dish) {
	fmt.Printf("Frying %s...\n", d.Name)
	d.IsFried = true
	if f.next != nil {
		f.next.Execute(d)
	}
}

func (f *FryingChef) SetNext(next Chef) {
	f.next = next
}
