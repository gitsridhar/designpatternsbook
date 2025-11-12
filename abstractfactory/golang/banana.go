package main

type Banana struct {
	name string
	Fruit
}

func (b *Banana) consume(name string) string {
	return "Consume " + b.name
}
