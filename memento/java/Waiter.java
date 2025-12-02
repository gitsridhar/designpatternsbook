package memento.java;

public class Waiter {
    private String state;

    public Waiter(String state) {
        this.state = state;
    }
    public String getState() {
        return state;
    }

    public Dish saveToMemento() {
        return new Dish(state);
    }

    public void restoreFromMemento(Dish dish) {
        this.state = dish.getState();
    }
}
