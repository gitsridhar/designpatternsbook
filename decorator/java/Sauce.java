package decorator.java;

public class Sauce implements Food {
    private final Food food;

    public Sauce(Food food) {
        this.food = food;
    }

    @Override
    public void dip() {
        food.dip();
        System.out.println("Adding sauce to the food");
    }
}

