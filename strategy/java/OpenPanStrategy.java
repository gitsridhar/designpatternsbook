package strategy.java;

public class OpenPanStrategy implements StrategyInterface {
    @Override
    public String performOperation(int a, int b) {
        return "Open Pan Strategy: " + (a + b);
    }
}
