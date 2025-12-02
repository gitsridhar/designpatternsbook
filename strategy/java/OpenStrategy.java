package strategy.java;

public class OpenStrategy extends Strategy {
    public String executeStrategy(int a, int b, StrategyInterface strategy) {
        System.out.println("Critical Strategy Executed");
        return strategy.performOperation(a, b);
    }
}
