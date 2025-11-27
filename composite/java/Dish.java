package composite.java;

public class Dish {
    Dish parentDish;

    void setParentDish(Dish parentDish) {
        this.parentDish = parentDish;
    }

    Dish getParentDish() {
        return parentDish;
    }

    void addDish(Dish dish) {
        // Default implementation (can be overridden by composite dishes)
        throw new UnsupportedOperationException("Cannot add dish to a leaf dish.");
    }

    void removeDish(Dish dish) {
        // Default implementation (can be overridden by composite dishes)
        throw new UnsupportedOperationException("Cannot remove dish from a leaf dish.");
    }
    boolean isComposite() {
        return false;
    }

    String prepare() {
        return "Preparing a simple dish.";
    }
}
