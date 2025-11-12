package main

type Potato struct {
	name string
	Vegetable
}

func (p *Potato) consume(name string) string {
	return "Consume " + p.name
}
