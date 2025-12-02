package visitor.java;

public class Visitor2 extends Visitor {
    @Override
    public void drink(Restaurant restaurant) {
        restaurant.serveDrink(this);
        restaurant.takePayment(this);
    }
}
