package strategy.java;

public class ClosedStrategy extends Strategy {
    @Override
    public String executeStrategy(int a, int b, StrategyInterface strategy) {
        System.out.println("Non-Critical Strategy Executed");
        return strategy.performOperation(a, b);
    }
}
