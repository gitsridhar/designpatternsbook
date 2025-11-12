package main

type Potato struct {
	IVegetable
}

func (p *Potato) consumeVegetable() string {
	return "Consuming Potato"
}
