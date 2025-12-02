package visitor.java;

public interface Restaurant {
    public void accept(Visitor visitor);

    public void serveDrink(Visitor visitor);

    public void takePayment(Visitor visitor);
}
