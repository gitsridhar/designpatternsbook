package main

import "fmt"

type FruitPrototype struct {
	Prototype
}

func (fp *FruitPrototype) clone() Prototype {
	fmt.Println("Cloning Fruit Prototype..")
	return &FruitPrototype{}
}
