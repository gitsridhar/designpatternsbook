package state.java;

public class EndOrderPhase extends OrderPhase {

    public void deliverOrder() {
        System.out.println("Order has already been delivered.");
    }

    public void startOrder() {
        System.out.println("Order has already been completed.");
    }

    public void endOrder() {
        System.out.println("Order is already completed and closed.");
        orderFood.switchOrderPhase(new ReadyOrderPhase());
    }
}
