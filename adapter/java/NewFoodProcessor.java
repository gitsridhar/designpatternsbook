package adapter.java;

public class NewFoodProcessor implements Chopper    {
    private FoodProcessor foodProcessor;

    public NewFoodProcessor(FoodProcessor foodProcessor) {
        this.foodProcessor = foodProcessor;
    }

    @Override
    public void chop() {
        foodProcessor.processFood();
        System.out.println("Chopping using Food Processor");
    }
}

