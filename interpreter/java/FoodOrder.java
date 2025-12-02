package interpreter.java;

import java.util.Map;

public class FoodOrder {
    private Map<String, String> orderItems;

    public Map<String, String> getOrderItems() {
        return orderItems;
    }

    public void setOrderItems(Map<String, String> orderItems) {
        this.orderItems = orderItems;
    }
}
