package main

// StrategyInterface defines the contract for all concrete strategies.
type StrategyInterface interface {
	PerformOperation(a, b int) int
}
