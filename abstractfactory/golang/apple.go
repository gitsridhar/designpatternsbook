package main

type Apple struct {
	name string
	Fruit
}

func (a *Apple) consume(name string) string {
	return "Consume " + a.name
}
