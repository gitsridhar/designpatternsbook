package state.java;

public class StartOrderPhase extends OrderPhase {

    public void deliverOrder() {
        System.out.println("Order is not yet prepared. Cannot deliver.");
    }

    public void startOrder() {
        System.out.println("Order is being started.");
    }

    public void endOrder() {
        System.out.println("Order is being prepared.");
        orderFood.switchOrderPhase(new EndOrderPhase());
    }
    
}
