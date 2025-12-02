package strategy.java;

public class Strategy {
    public String executeStrategy(int a, int b, StrategyInterface strategy) {
        return strategy.performOperation(a, b);
    }
}
