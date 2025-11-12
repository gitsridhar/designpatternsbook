package main

type FruitConsumer struct {
	IConsumer
}

func (fc *FruitConsumer) consume() string {
	apple := Apple{}
	banana := Banana{}
	return apple.consumeFruit() + "," + banana.consumeFruit()
}
