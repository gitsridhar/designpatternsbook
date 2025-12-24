package main

// Strategy acts as the context that triggers the interface methods.
type Strategy struct{}

// ExecuteStrategy takes the interface as a parameter to perform the operation.
func (s *Strategy) ExecuteStrategy(a, b int, si StrategyInterface) int {
	return si.PerformOperation(a, b)
}

// OpenStrategy extends the base Strategy context for specific use cases.
type OpenStrategy struct {
	Strategy
}

// ClosedStrategy extends the base Strategy context for specific use cases.
type ClosedStrategy struct {
	Strategy
}
