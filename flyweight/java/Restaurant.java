package flyweight.java;

import java.util.List;

public class Restaurant {
    private List<Food> foods = new java.util.ArrayList<Food>();

    public void addFood(int cost, String name, String cuisine, String category) {
        FoodType foodType = FoodFactory.getFoodType(cuisine, category);
        Food food = new Food(cost, name, foodType);
        foods.add(food);
    }

    public void serveFood() {
        for (Food food : foods) {
            food.consume();
        }
    }
}