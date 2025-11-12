package main

type Apple struct {
	IFruit
}

func (a *Apple) consumeFruit() string {
	return "Consuming Apple"
}
