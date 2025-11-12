package main

type Banana struct {
	IFruit
}

func (b *Banana) consumeFruit() string {
	return "Consuming Banana"
}
