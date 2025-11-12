package main

type IMyFactory interface {
	consumeFruit() IFruit
	consumeVegetable() IVegetable
}
