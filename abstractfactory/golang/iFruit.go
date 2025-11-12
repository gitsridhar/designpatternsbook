package main

type IFruit interface {
	consume(string) string
}

type Fruit struct {
	name string
}
