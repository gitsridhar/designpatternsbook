package flyweight.java;

public class Food {
    private int cost;
    private String name;
    private FoodType foodType;

    public Food(int cost, String name, FoodType foodType) {
        this.cost = cost;
        this.name = name;
        this.foodType = foodType;
    }

    public void consume() {
        System.out.println("Consuming " + name + " of type " + 
            foodType + " which costs " + cost);
        System.out.println("Food Type Consuming details: ");
        foodType.consume();
    }
}
