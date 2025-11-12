package main

type Beans struct {
	name string
	Vegetable
}

func (b *Beans) consume(name string) string {
	return "Consume " + b.name
}
