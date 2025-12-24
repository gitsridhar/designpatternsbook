package main

import "fmt"

func main() {
	// Initialize Strategies
	openPan := &OpenPanStrategy{}
	closedPan := &ClosedPanStrategy{}

	// Initialize Contexts
	openCtx := &OpenStrategy{}
	closedCtx := &ClosedStrategy{}

	// Execute Open Strategy
	res1 := openCtx.ExecuteStrategy(10, 5, openPan)
	fmt.Printf("Result: %d\n\n", res1)

	// Execute Closed Strategy
	res2 := closedCtx.ExecuteStrategy(10, 5, closedPan)
	fmt.Printf("Result: %d\n", res2)
}
