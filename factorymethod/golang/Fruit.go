package main

import "fmt"

type Fruit struct {
	name string
}

func (f *Fruit) consume() string {
	cons := "Consume Apple : " + f.name
	fmt.Println(cons)
	return cons
}
