package facade.java;

public class MyFacade {
    public static void main(String[] args) {
        ColdFood coldFood = new ColdFood();
        HotFood hotFood = new HotFood();
        Restaurant restaurant = new Restaurant(coldFood, hotFood);
        String result = restaurant.operation();
        System.out.println(result);
    }
}
