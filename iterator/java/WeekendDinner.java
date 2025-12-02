package iterator.java;

import java.util.List;

public class WeekendDinner extends Dinner {
    private List<Dish> dishes;
    
    public WeekendDinner(List<Dish> dishes) {
        this.dishes = dishes;
    }
    public Eating createDinner() {
        return new RestaurantEating(dishes);
    }
}
