package flyweight.java;

public class MyFlyweight {
    public static void main(String[] args) {
        Restaurant restaurant = new Restaurant();

        restaurant.addFood(10, "Pasta", "Italian", "Main Course");
        restaurant.addFood(8, "Bruschetta", "Italian", "Appetizer");
        restaurant.addFood(12, "Tiramisu", "Italian", "Dessert");
        restaurant.addFood(15, "Sushi", "Japanese", "Main Course");
        restaurant.addFood(7, "Miso Soup", "Japanese", "Appetizer");
        restaurant.addFood(9, "Mochi", "Japanese", "Dessert");

        restaurant.serveFood();
    }
}
