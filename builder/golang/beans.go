package main

type Beans struct {
	IVegetable
}

func (b *Beans) consumeVegetable() string {
	return "Consuming Beans"
}
