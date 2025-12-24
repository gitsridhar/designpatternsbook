package main

type Chef interface {
	Execute(*Dish)
	SetNext(Chef)
}

type Dish struct {
	Name             string
	IngredientsReady bool
	IsBoiled         bool
	IsFried          bool
	IsPlated         bool
}
