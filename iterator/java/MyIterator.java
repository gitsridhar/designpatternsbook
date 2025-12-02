package iterator.java;

import java.util.List;

public class MyIterator {
    public static void main(String[] args) {
        // Example usage
        Dish dish1 = new Dish("Pasta");
        Dish dish2 = new Dish("Salad");
        List<Dish> dishes = new java.util.ArrayList<>();
        dishes.add(dish1);
        dishes.add(dish2);
        
        WeekendDinner weekendDinner = new WeekendDinner(dishes);
        Eating eating = weekendDinner.createDinner();

        while (eating.hasNextDish()) {
            Dish dish = eating.nextDish();
            eating.eat(dish);
        }
    }
}
