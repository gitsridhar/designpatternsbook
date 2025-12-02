package interpreter.java;

import java.util.HashMap;
import java.util.Map;

public class MyInterpreter {
    public static void main(String[] args) {
        FoodOrder order = new FoodOrder();
        Map<String, String> items = new HashMap<>();
        items.put("Drink", "Coke, Large");
        items.put("Food", "Burger, Medium");
        order.setOrderItems(items); 

        Item allFood = new AllFood(
            new DrinkItem("Coke", "Large"),
            new FoodItem("Burger", "Medium")
        );

        allFood.interpret(order);
    }
}
