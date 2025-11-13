package main

import (
	"fmt"
	"sync"
)

var lock = &sync.Mutex{}

// Alternate, use sync.Once and once.Do,etc.

type Kitchen struct {
}

var kitchenInstance *Kitchen

func getKitchen(wg *sync.WaitGroup) *Kitchen {
	defer wg.Done()
	if kitchenInstance == nil {
		lock.Lock()
		defer lock.Unlock()
		if kitchenInstance == nil {
			fmt.Println("Creating the kitchen")
			kitchenInstance = &Kitchen{}
		}
	}
	fmt.Println("Returnig the kitchen")
	return kitchenInstance
}
