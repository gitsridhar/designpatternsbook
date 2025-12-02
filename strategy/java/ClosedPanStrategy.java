package strategy.java;

public class ClosedPanStrategy implements StrategyInterface {
    @Override
    public String performOperation(int a, int b) {
        return "Closed Pan Strategy: " + (a * b);
    }
    
}
