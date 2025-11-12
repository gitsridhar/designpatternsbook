package main

type IVegetable interface {
	consume(string) string
}

type Vegetable struct {
	name string
}
