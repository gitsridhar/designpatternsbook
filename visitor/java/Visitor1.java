package visitor.java;

public class Visitor1 extends Visitor {
    @Override
    public void drink(Restaurant restaurant) {
        restaurant.serveDrink(this);
        restaurant.takePayment(this);
    }
}
