package strategy.java;

public class FoodPreparation {
    private StrategyInterface strategy;

    public void setStrategy(StrategyInterface strategy) {
        this.strategy = strategy;
    }

    public void prepareFood() {
        if (strategy != null) {
            int a = 1; // Example input
            int b = 2; // Example input
            System.out.println(strategy.performOperation(a, b));
        } else {
            System.out.println("No preparation strategy set.");
        }
    }
}
