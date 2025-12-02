package state.java;

public class MyState {
    public static void main(String[] args) {
        OrderFood orderFood = new OrderFood(new StartOrderPhase());

        orderFood.startOrder();
        orderFood.deliverOrder();
        orderFood.endOrder();
        orderFood.endOrder(); // Trying to end the order again
    }
}
