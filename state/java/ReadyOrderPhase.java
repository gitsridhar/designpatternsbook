package state.java;

public class ReadyOrderPhase extends OrderPhase {

    public void deliverOrder() {
        System.out.println("Order is being delivered.");
    }

    public void startOrder() {
        System.out.println("Order is being started.");
    }

    public void endOrder() {
        System.out.println("Order is already prepared and ready for delivery.");
    }   

}
