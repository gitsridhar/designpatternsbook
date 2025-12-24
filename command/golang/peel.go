package main

import "fmt"

// Peel is the receiver that knows how to perform the work
type Peel struct {
	Item string
}

func (p *Peel) DoIt() {
	fmt.Printf("Receiver: Peeling the %s...\n", p.Item)
}
