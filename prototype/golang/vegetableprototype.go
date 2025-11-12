package main

import "fmt"

type VegetablePrototype struct {
	Prototype
}

func (fp *VegetablePrototype) clone() Prototype {
	fmt.Println("Cloning Vegetable Prototype..")
	return &VegetablePrototype{}
}
