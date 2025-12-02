package strategy.java;

public class MyStrategy {
    public static void main(String[] args) {
        FoodPreparation foodPreparation = new FoodPreparation();

        // Using Critical Strategy
        foodPreparation.setStrategy(new OpenPanStrategy());
        foodPreparation.prepareFood();

        // Using Non-Critical Strategy
        foodPreparation.setStrategy(new ClosedPanStrategy());
        foodPreparation.prepareFood();
    }
}
