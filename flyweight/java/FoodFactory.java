package flyweight.java;

import java.util.Map;

public class FoodFactory {
    static Map<String, FoodType> foodTypes = new java.util.HashMap<String, FoodType>();

    public static FoodType getFoodType(String cuisine, String category) {
        String key = cuisine + "-" + category;
        if (!foodTypes.containsKey(key)) {
            foodTypes.put(key, new FoodType(cuisine, category));
        }
        return foodTypes.get(key);
    }
}