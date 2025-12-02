package visitor.java;

public class Restaurant2 implements Restaurant {
    @Override
    public void accept(Visitor visitor) {
        visitor.drink(this);
    }

    @Override
    public void serveDrink(Visitor visitor) {
        System.out.println("Restaurant2 is serving drink to visitor.");
    }

    @Override
    public void takePayment(Visitor visitor) {
        System.out.println("Restaurant2 is taking payment from visitor.");
    }
}
