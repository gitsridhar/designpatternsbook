package main

type VegetableConsumer struct {
	IConsumer
}

func (fc *VegetableConsumer) consume() string {
	potato := Potato{}
	beans := Beans{}
	return potato.consumeVegetable() + "," + beans.consumeVegetable()
}
