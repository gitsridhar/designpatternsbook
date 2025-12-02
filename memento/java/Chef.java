package memento.java;

import java.util.ArrayList;
import java.util.List;

public class Chef {
    private List<Dish> dishes = new ArrayList<>();
    private Waiter waiter;
 
    public Chef(Waiter waiter) {
        this.waiter = waiter;
    }

    public void backup() {
        dishes.add(waiter.saveToMemento());
    }

    public void undo() {
        if (dishes.size() > 0) {
            Dish dish = dishes.remove(dishes.size() - 1);
            waiter.restoreFromMemento(dish);
        }
    }
}

