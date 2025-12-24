package main

import "fmt"

// OpenPanStrategy represents one specific algorithm.
type OpenPanStrategy struct{}

func (s *OpenPanStrategy) PerformOperation(a, b int) int {
	fmt.Println("Executing Open Pan Strategy (Addition)")
	return a + b
}

// ClosedPanStrategy represents an alternative algorithm.
type ClosedPanStrategy struct{}

func (s *ClosedPanStrategy) PerformOperation(a, b int) int {
	fmt.Println("Executing Closed Pan Strategy (Multiplication)")
	return a * b
}
