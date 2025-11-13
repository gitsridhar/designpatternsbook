package main

import (
	"fmt"
	"sync"
)

func main() {
	var wg sync.WaitGroup

	for range 7 {
		fmt.Println("Startig Threads")
		wg.Add(1)
		go getKitchen(&wg)
	}
	wg.Wait()
}
