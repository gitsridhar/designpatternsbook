package state.java;

public class OrderFood {
    private OrderPhase orderPhase;

    public OrderFood(OrderPhase orderPhase) {
        this.orderPhase = orderPhase;
        this.switchOrderPhase(orderPhase);
    }

    public void switchOrderPhase(OrderPhase orderPhase) {
        this.orderPhase.setOrderPhase(this);
    }

    public void startOrder() {
        this.orderPhase.startOrder();
    }

    public void endOrder() {
        this.orderPhase.endOrder();
    }

    public void deliverOrder() {
        this.orderPhase.deliverOrder();
    }
}
