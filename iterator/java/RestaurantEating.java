package iterator.java;

import java.util.List;

public class RestaurantEating extends Eating {

    private List<Dish> dishes;
    private int currentIndex = 0;

    public RestaurantEating(List<Dish> dishes) {
        this.dishes = dishes;
    }

    @Override
    public boolean hasNextDish() {
        if (currentIndex < dishes.size()) {
            return true;
        }
        return false;
    }

    @Override
    public Dish nextDish() {
        if (!hasNextDish()) {
            throw new IndexOutOfBoundsException("No more dishes available.");
        }
        return dishes.get(currentIndex++);
    }
}
