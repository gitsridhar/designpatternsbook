package composite.java;

import java.util.ArrayList;
import java.util.List;

public class Serving extends Dish {

    List <Dish> dishes = new ArrayList<>();
  
    public void addDish(Dish dish) {
        dishes.add(dish);
    }
    public void removeDish(Dish dish) {
        dishes.remove(dish);
    }
    public List<Dish> getDishes() {
        return dishes;
    }

    @Override
    boolean isComposite() { 
        return true; 
    }

    @Override
    String prepare() {
        StringBuilder result = new StringBuilder("");
        for (Dish dish : dishes) {
            result.append(dish.prepare()).append("\n");
        }
        return result.toString().trim();
    }
}
