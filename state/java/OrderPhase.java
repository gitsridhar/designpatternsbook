package state.java;

public class OrderPhase {
    protected OrderFood orderFood;

    public void setOrderPhase(OrderFood orderFood) {
        this.orderFood = orderFood;
    }

    public void startOrder() {}
    public void endOrder() {}
    public void deliverOrder() {}

}
