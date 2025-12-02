package visitor.java;

public class MyVisitor {
    public static void main(String[] args) {
        Restaurant restaurant = new Restaurant1();

        Visitor visitor1 = new Visitor1();
        restaurant.accept(visitor1);
        visitor1.drink(restaurant);

        Visitor visitor2 = new Visitor2();
        restaurant.accept(visitor2);
        visitor2.drink(restaurant);
    }
}
