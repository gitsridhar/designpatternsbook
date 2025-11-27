package adapter.java;

public class MyAdapter {
    public static void main(String[] args) {
        FoodProcessor foodProcessor = new FoodProcessor();

        Chopper chopper = new NewFoodProcessor(foodProcessor);
        chopper.chop();
    }
}
