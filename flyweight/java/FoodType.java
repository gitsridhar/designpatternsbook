package flyweight.java;

public class FoodType {
    private String cuisine;
    private String category;

    public FoodType(String cuisine, String category) {
        this.cuisine = cuisine;
        this.category = category;
    }

    @Override
    public String toString() {
        return "Cuisine: " + cuisine + ", Category: " + category;
    }

    public void consume() {
        System.out.println("Consuming food of type " + this);
    }
}
