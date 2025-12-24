package main

import "fmt"

// OrderPhase is the State interface
type OrderPhase interface {
	AddItem()
	Confirm()
}

// BasePhase provides default "invalid action" messages
type BasePhase struct{}

func (b *BasePhase) AddItem() {
	fmt.Println("Action not available in this phase.")
}

func (b *BasePhase) Confirm() {
	fmt.Println("Action not available in this phase.")
}
